package io.provenance.bilateral.contract

import io.provenance.bilateral.execute.Ask.ScopeTradeAsk
import io.provenance.bilateral.execute.Bid.CoinTradeBid
import io.provenance.bilateral.execute.Bid.ScopeTradeBid
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import io.provenance.bilateral.execute.ExecuteMatch
import io.provenance.bilateral.execute.UpdateAsk
import io.provenance.bilateral.execute.UpdateBid
import io.provenance.bilateral.models.RequestDescriptor
import io.provenance.metadata.v1.PartyType
import io.provenance.metadata.v1.ScopeRequest
import io.provenance.scope.util.MetadataAddress
import mu.KLogging
import org.junit.jupiter.api.Test
import testconfiguration.ContractIntTest
import testconfiguration.extensions.getBalance
import testconfiguration.extensions.testGetCoinTrade
import testconfiguration.extensions.testGetScopeTrade
import testconfiguration.functions.assertSingle
import testconfiguration.functions.assertSucceeds
import testconfiguration.functions.giveTestDenom
import testconfiguration.functions.newCoin
import testconfiguration.functions.newCoins
import testconfiguration.util.ScopeWriteUtil
import java.time.OffsetDateTime
import java.util.UUID
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class ScopeTradeIntTest : ContractIntTest() {
    private companion object : KLogging()

    @Test
    fun testScopeTradeFullFlow() {
        val (scopeUuid, _) = ScopeWriteUtil.writeMockScope(
            pbClient = pbClient,
            signer = asker,
            ownerAddress = contractInfo.contractAddress,
            valueOwnerAddress = contractInfo.contractAddress,
        )
        val bidderDenom = "scopetradefullflow"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(500, bidderDenom),
            receiverAddress = bidder.address(),
        )
        val askUuid = UUID.randomUUID()
        logger.info("Creating scope trade ask [$askUuid]")
        createAsk(
            createAsk = CreateAsk(
                ask = ScopeTradeAsk(
                    id = askUuid.toString(),
                    scopeAddress = MetadataAddress.forScope(scopeUuid).toString(),
                    quote = newCoins(500, bidderDenom),
                ),
                descriptor = RequestDescriptor("Example description", OffsetDateTime.now()),
            )
        )
        val bidUuid = UUID.randomUUID()
        logger.info("Creating scope trade bid [$bidUuid]")
        createBid(
            createBid = CreateBid(
                bid = ScopeTradeBid(
                    id = bidUuid.toString(),
                    scopeAddress = MetadataAddress.forScope(scopeUuid).toString(),
                    quote = newCoins(500, bidderDenom),
                ),
                descriptor = RequestDescriptor("Example description", OffsetDateTime.now()),
            ),
        )
        logger.info("Executing match for ask [$askUuid] and bid [$bidUuid]")
        val executeMatchResponse = executeMatch(ExecuteMatch(askUuid.toString(), bidUuid.toString()))
        assertTrue(
            actual = executeMatchResponse.askDeleted,
            message = "Expected the execute match response to indicate that the ask was deleted",
        )
        assertTrue(
            actual = executeMatchResponse.bidDeleted,
            message = "Expected the execute match response to indicate that the bid was deleted",
        )
        assertTrue(
            actual = executeMatchResponse.collateralReleased,
            message = "Collateral should always be released for scope trades",
        )
        assertEquals(
            expected = 500L,
            actual = pbClient.getBalance(asker.address(), bidderDenom),
            message = "The asker should have received the bidder's [$bidderDenom] after the trade was completed",
        )
        val scopeInfo = pbClient.metadataClient.scope(ScopeRequest.newBuilder().setScopeId(scopeUuid.toString()).build())
        assertEquals(
            expected = bidder.address(),
            actual = scopeInfo.scope.scope.ownersList.assertSingle("Expected a single owner to be set on the scope") { party ->
                party.role == PartyType.PARTY_TYPE_OWNER
            }.address,
            message = "The bidder to now be marked as the only owner on the scope",
        )
        assertEquals(
            expected = bidder.address(),
            actual = scopeInfo.scope.scope.valueOwnerAddress,
            message = "Expected the bidder to now be the value owner of the scope",
        )
        assertEquals(
            expected = 0L,
            actual = pbClient.getBalance(bidder.address(), bidderDenom),
            message = "The bidder should no longer have any of its test denom [$bidderDenom]",
        )
    }

    @Test
    fun testCancelAsk() {
        val (scopeUuid, _) = ScopeWriteUtil.writeMockScope(
            pbClient = pbClient,
            signer = asker,
            ownerAddress = contractInfo.contractAddress,
            valueOwnerAddress = contractInfo.contractAddress,
        )
        val askUuid = UUID.randomUUID()
        val createResponse = createAsk(
            createAsk = CreateAsk(
                ask = ScopeTradeAsk(
                    id = askUuid.toString(),
                    scopeAddress = MetadataAddress.forScope(scopeUuid).toString(),
                    quote = newCoins(5000, "nhash"),
                ),
            )
        )
        val cancelResponse = cancelAsk(askUuid.toString())
        assertEquals(
            expected = createResponse.askOrder,
            actual = cancelResponse.cancelledAskOrder,
            message = "Expected the cancelled ask order to be included in the cancel ask response",
        )
        assertTrue(
            actual = cancelResponse.collateralReleased,
            message = "The collateral should always be released for cancelled scope trades",
        )
        val scopeInfo = pbClient.metadataClient.scope(ScopeRequest.newBuilder().setScopeId(scopeUuid.toString()).build())
        assertEquals(
            expected = asker.address(),
            actual = scopeInfo.scope.scope.ownersList.assertSingle("Expected a single owner to be set on the scope") { party ->
                party.role == PartyType.PARTY_TYPE_OWNER
            }.address,
            message = "The asker to now be marked as the only owner on the scope after the ask's cancellation",
        )
        assertEquals(
            expected = asker.address(),
            actual = scopeInfo.scope.scope.valueOwnerAddress,
            message = "Expected the asker to now be the value owner of the scope after the ask's cancellatiion",
        )
    }

    @Test
    fun testCancelBid() {
        val bidderDenom = "testscopetradecancelbid"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(10000, bidderDenom),
            receiverAddress = bidder.address(),
        )
        val bidUuid = UUID.randomUUID()
        val createResponse = createBid(
            createBid = CreateBid(
                bid = ScopeTradeBid(
                    id = bidUuid.toString(),
                    scopeAddress = MetadataAddress.forScope(UUID.randomUUID()).toString(),
                    quote = newCoins(10000, bidderDenom),
                ),
            )
        )
        assertEquals(
            expected = 0L,
            actual = pbClient.getBalance(bidder.address(), bidderDenom),
            message = "Expected the bidder's [$bidderDenom] coin to be held in escrow when the bid is created",
        )
        val cancelResponse = cancelBid(bidUuid.toString())
        assertEquals(
            expected = createResponse.bidOrder,
            actual = cancelResponse.cancelledBidOrder,
            message = "Expected the cancelled bid order to be included in the cancel bid response",
        )
        assertEquals(
            expected = 10000L,
            actual = pbClient.getBalance(bidder.address(), bidderDenom),
            message = "The bidder's entire [$bidderDenom] quote balance should be returned when the bid is cancelled",
        )
    }

    @Test
    fun testUpdateAsk() {
        val (scopeUuid, _) = ScopeWriteUtil.writeMockScope(
            pbClient = pbClient,
            signer = asker,
            ownerAddress = contractInfo.contractAddress,
            valueOwnerAddress = contractInfo.contractAddress,
        )
        val quoteDenom = "updatescopetradeaskquote"
        val askUuid = UUID.randomUUID()
        assertSucceeds("Expected creating a scope trade ask to succeed") {
            createAsk(
                createAsk = CreateAsk(
                    ask = ScopeTradeAsk(
                        id = askUuid.toString(),
                        scopeAddress = MetadataAddress.forScope(scopeUuid).toString(),
                        quote = newCoins(500, quoteDenom),
                    ),
                    descriptor = RequestDescriptor("Example description", OffsetDateTime.now()),
                ),
            )
        }
        val response = assertSucceeds("Expected updating a scope trade ask to succeed") {
            updateAsk(
                updateAsk = UpdateAsk(
                    ask = ScopeTradeAsk(
                        id = askUuid.toString(),
                        scopeAddress = MetadataAddress.forScope(scopeUuid).toString(),
                        quote = newCoins(1200, quoteDenom),
                    ),
                    descriptor = RequestDescriptor("Example description", OffsetDateTime.now()),
                ),
                signer = asker,
            )
        }
        assertEquals(
            expected = newCoins(1200, quoteDenom),
            actual = response.updatedAskOrder.testGetScopeTrade().quote,
            message = "Expected the updated ask order to include the new quote",
        )
    }

    @Test
    fun testUpdateBidSameType() {
        val quoteDenom = "updatescopetradebidquote"
        val quoteDenom2 = "updatescopetradebidquote2"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, quoteDenom),
            receiverAddress = bidder.address(),
        )
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, quoteDenom2),
            receiverAddress = bidder.address(),
        )
        val bidUuid = UUID.randomUUID()
        val scopeAddress = MetadataAddress.forScope(UUID.randomUUID()).toString()
        assertSucceeds("Expected creating a scope trade bid to succeed") {
            createBid(
                createBid = CreateBid(
                    bid = ScopeTradeBid(
                        id = bidUuid.toString(),
                        scopeAddress = scopeAddress,
                        quote = newCoins(100, quoteDenom),
                    ),
                    descriptor = RequestDescriptor("Example description", OffsetDateTime.now()),
                ),
            )
        }
        assertEquals(
            expected = 900,
            actual = pbClient.getBalance(bidder.address(), quoteDenom),
            message = "The bid should have removed the correct amount of quote funds from the bidder account",
        )
        val response = assertSucceeds("Expected updating a scope trade bid to succeed") {
            updateBid(
                updateBid = UpdateBid(
                    bid = ScopeTradeBid(
                        id = bidUuid.toString(),
                        scopeAddress = scopeAddress,
                        quote = newCoins(300, quoteDenom2),
                    ),
                    descriptor = RequestDescriptor("Example description", OffsetDateTime.now()),
                ),
            )
        }
        assertEquals(
            expected = newCoins(300, quoteDenom2),
            actual = response.updatedBidOrder.testGetScopeTrade().quote,
            message = "Expected the updated bid order to include the new quote",
        )
        assertEquals(
            expected = 1000,
            actual = pbClient.getBalance(bidder.address(), quoteDenom),
            message = "The original bid quote denom should be refunded to the bidder",
        )
        assertEquals(
            expected = 700,
            actual = pbClient.getBalance(bidder.address(), quoteDenom2),
            message = "The bid should have debited the new quote denom from the bidder",
        )
    }

    @Test
    fun testUpdateBidNewType() {
        val quoteDenom = "updatescopetradenewtypebidquote"
        val quoteDenom2 = "updatescopetradenewtypebidquote2"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, quoteDenom),
            receiverAddress = bidder.address(),
        )
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, quoteDenom2),
            receiverAddress = bidder.address(),
        )
        val bidUuid = UUID.randomUUID()
        val scopeAddress = MetadataAddress.forScope(UUID.randomUUID()).toString()
        assertSucceeds("Expected creating a scope trade bid to succeed") {
            createBid(
                createBid = CreateBid(
                    bid = ScopeTradeBid(
                        id = bidUuid.toString(),
                        scopeAddress = scopeAddress,
                        quote = newCoins(100, quoteDenom),
                    ),
                    descriptor = RequestDescriptor("Example description", OffsetDateTime.now()),
                ),
            )
        }
        assertEquals(
            expected = 900,
            actual = pbClient.getBalance(bidder.address(), quoteDenom),
            message = "The bid should have removed the correct amount of quote funds from the bidder account",
        )
        val response = assertSucceeds("Expected updating a scope trade bid to succeed") {
            updateBid(
                updateBid = UpdateBid(
                    bid = CoinTradeBid(
                        id = bidUuid.toString(),
                        quote = newCoins(300, quoteDenom2),
                        base = newCoins(1, "some base"),
                    ),
                    descriptor = RequestDescriptor("Example description", OffsetDateTime.now()),
                ),
            )
        }
        val collateral = response.updatedBidOrder.testGetCoinTrade()
        assertEquals(
            expected = newCoins(300, quoteDenom2),
            actual = collateral.quote,
            message = "Expected the updated bid order to include the new quote",
        )
        assertEquals(
            expected = newCoins(1, "some base"),
            actual = collateral.base,
            message = "Expected the updated bid order to include the new base",
        )
        assertEquals(
            expected = 1000,
            actual = pbClient.getBalance(bidder.address(), quoteDenom),
            message = "The original bid quote denom should be refunded to the bidder",
        )
        assertEquals(
            expected = 700,
            actual = pbClient.getBalance(bidder.address(), quoteDenom2),
            message = "The bid should have debited the new quote denom from the bidder",
        )
    }

    @Test
    fun testGetAsksByCollateralId() {
        val scopeAddress = ScopeWriteUtil.writeMockScope(
            pbClient = pbClient,
            signer = asker,
            ownerAddress = contractInfo.contractAddress,
            valueOwnerAddress = contractInfo.contractAddress,
        ).let { (scopeUuid, _) -> MetadataAddress.forScope(scopeUuid).toString() }
        val createAskResponse = createAsk(
            createAsk = CreateAsk(
                ask = ScopeTradeAsk(
                    id = UUID.randomUUID().toString(),
                    scopeAddress = scopeAddress,
                    quote = newCoins(5000, "nhash"),
                )
            )
        )
        val queryAsk = bilateralClient.getAsksByCollateralId(scopeAddress).assertSingle("A single ask should be returned by the query")
        assertEquals(
            expected = createAskResponse.askOrder,
            actual = queryAsk,
            message = "The correct ask should be returned by the query",
        )
    }
}
