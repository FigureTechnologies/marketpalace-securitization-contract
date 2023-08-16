package io.provenance.bilateral.contract

import io.provenance.bilateral.execute.Ask.CoinTradeAsk
import io.provenance.bilateral.execute.Bid.CoinTradeBid
import io.provenance.bilateral.execute.Bid.ScopeTradeBid
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import io.provenance.bilateral.execute.ExecuteMatch
import io.provenance.bilateral.execute.UpdateAsk
import io.provenance.bilateral.execute.UpdateBid
import io.provenance.bilateral.models.AdminMatchOptions
import io.provenance.bilateral.models.AttributeRequirement
import io.provenance.bilateral.models.RequestDescriptor
import io.provenance.bilateral.models.enums.AttributeRequirementType
import org.junit.jupiter.api.Test
import testconfiguration.ContractIntTest
import testconfiguration.extensions.getBalance
import testconfiguration.extensions.getBalanceMap
import testconfiguration.extensions.testGetCoinTrade
import testconfiguration.extensions.testGetScopeTrade
import testconfiguration.functions.assertSingle
import testconfiguration.functions.assertSucceeds
import testconfiguration.functions.giveTestDenom
import testconfiguration.functions.newCoin
import testconfiguration.functions.newCoins
import java.time.OffsetDateTime
import java.util.UUID
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlin.test.assertNull
import kotlin.test.assertTrue

class CoinTradeIntTest : ContractIntTest() {
    @Test
    fun testCoinTradeCompleteFlow() {
        val askerDenom = "cointradecompleteflowa"
        val bidderDenom = "cointradecompleteflowb"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, askerDenom),
            receiverAddress = asker.address(),
        )
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, bidderDenom),
            receiverAddress = bidder.address(),
        )
        val quote = newCoins(1000, bidderDenom)
        val base = newCoins(1000, askerDenom)
        val askUuid = UUID.randomUUID()
        assertSucceeds("Ask should be created without error") {
            createAsk(
                CreateAsk(
                    ask = CoinTradeAsk(
                        id = askUuid.toString(),
                        base = base,
                        quote = quote,
                    ),
                    descriptor = RequestDescriptor(
                        description = "Example description",
                        effectiveTime = OffsetDateTime.now(),
                        attributeRequirement = AttributeRequirement(listOf("a.pb", "b.pb"), AttributeRequirementType.NONE),
                    )
                )
            )
        }
        assertEquals(
            expected = 0,
            actual = pbClient.getBalance(asker.address(), askerDenom),
            message = "The asker account's entire coin balance should be held in escrow after creating an ask",
        )
        val bidUuid = UUID.randomUUID()
        assertSucceeds("Bid should be created without error") {
            createBid(
                createBid = CreateBid(
                    bid = CoinTradeBid(
                        id = bidUuid.toString(),
                        base = base,
                        quote = quote,
                    ),
                    descriptor = RequestDescriptor(
                        description = "Example description",
                        effectiveTime = OffsetDateTime.now(),
                        attributeRequirement = AttributeRequirement(listOf("c.pb"), AttributeRequirementType.NONE),
                    ),
                )
            )
        }
        assertEquals(
            expected = 0,
            actual = pbClient.getBalance(bidder.address(), bidderDenom),
            message = "The bidder account's entire coin balance should be held in escrow after creating a bid",
        )
        val executeMatchResponse = assertSucceeds("Match should be executed without error") {
            executeMatch(executeMatch = ExecuteMatch(askId = askUuid.toString(), bidId = bidUuid.toString()))
        }
        assertTrue(
            actual = executeMatchResponse.askDeleted,
            message = "Expected the response to indicate that the ask was deleted",
        )
        assertTrue(
            actual = executeMatchResponse.bidDeleted,
            message = "Expected the response to indicate that the bid was deleted",
        )
        assertTrue(
            actual = executeMatchResponse.collateralReleased,
            message = "The collateral released flag should always be true for a coin trade",
        )
        val askerBalances = pbClient.getBalanceMap(asker.address())
        assertNull(
            actual = askerBalances[askerDenom],
            message = "The asker should no longer have any [$askerDenom] because it should have been sent to the bidder",
        )
        assertEquals(
            expected = 1000L,
            actual = askerBalances[bidderDenom],
            message = "The asker should have received all of the bidder's [$bidderDenom]",
        )
        val bidderBalances = pbClient.getBalanceMap(bidder.address())
        assertNull(
            actual = bidderBalances[bidderDenom],
            message = "The bidder should no longer have any [$bidderDenom] because it should have been sent to the asker",
        )
        assertEquals(
            expected = 1000L,
            actual = bidderBalances[askerDenom],
            message = "The bidder should have received all of the asker's [$askerDenom]"
        )
    }

    @Test
    fun testMismatchedBidExecute() {
        val askerDenom = "cointrademismatchedask"
        val bidderDenom = "cointrademismatchedbid"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, askerDenom),
            receiverAddress = asker.address(),
        )
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, bidderDenom),
            receiverAddress = bidder.address(),
        )
        val askQuote = newCoins(1000, "someotherthing")
        val bidQuote = newCoins(1000, bidderDenom)
        val base = newCoins(1000, askerDenom)
        val askUuid = UUID.randomUUID()
        assertSucceeds("Ask should be created without error") {
            createAsk(
                createAsk = CreateAsk(
                    ask = CoinTradeAsk(
                        id = askUuid.toString(),
                        base = base,
                        quote = askQuote,
                    ),
                    descriptor = RequestDescriptor(
                        description = "Example description",
                        effectiveTime = OffsetDateTime.now(),
                        attributeRequirement = AttributeRequirement(listOf("a.pb", "b.pb"), AttributeRequirementType.NONE),
                    ),
                )
            )
        }
        assertEquals(
            expected = 0,
            actual = pbClient.getBalance(asker.address(), askerDenom),
            message = "The asker account's entire coin balance should be held in escrow after creating an ask",
        )
        val bidUuid = UUID.randomUUID()
        assertSucceeds("Bid should be created without error") {
            createBid(
                createBid = CreateBid(
                    bid = CoinTradeBid(
                        id = bidUuid.toString(),
                        base = base,
                        quote = bidQuote,
                    ),
                    descriptor = RequestDescriptor(
                        description = "Example description",
                        effectiveTime = OffsetDateTime.now(),
                        attributeRequirement = AttributeRequirement(listOf("c.pb"), AttributeRequirementType.NONE),
                    ),
                )
            )
        }
        assertEquals(
            expected = 0,
            actual = pbClient.getBalance(bidder.address(), bidderDenom),
            message = "The bidder account's entire coin balance should be held in escrow after creating a bid",
        )
        val executeMatch = ExecuteMatch(askUuid.toString(), bidUuid.toString())
        assertFails("Match should fail because the quotes don't match") {
            executeMatch(executeMatch = executeMatch)
        }
        val matchResponse = assertSucceeds("Match should succeed because it was manually allowed") {
            executeMatch(executeMatch = executeMatch.copy(adminMatchOptions = AdminMatchOptions.CoinTradeAdminOptions(acceptMismatchedBids = true)))
        }
        assertTrue(actual = matchResponse.askDeleted, message = "The ask should be deleted")
        assertTrue(actual = matchResponse.bidDeleted, message = "The bid should be deleted")
        val askerBalances = pbClient.getBalanceMap(asker.address())
        assertNull(
            actual = askerBalances[askerDenom],
            message = "The asker should no longer have any [$askerDenom] because it should have been sent to the bidder",
        )
        assertEquals(
            expected = 1000L,
            actual = askerBalances[bidderDenom],
            message = "The asker should have received all of the bidder's [$bidderDenom]",
        )
        val bidderBalances = pbClient.getBalanceMap(bidder.address())
        assertNull(
            actual = bidderBalances[bidderDenom],
            message = "The bidder should no longer have any [$bidderDenom] because it should have been sent to the asker",
        )
        assertEquals(
            expected = 1000L,
            actual = bidderBalances[askerDenom],
            message = "The bidder should have received all of the asker's [$askerDenom]"
        )
    }

    @Test
    fun testCancelAsk() {
        val askerDenom = "cointradecancelask"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(amount = 100, denom = askerDenom),
            receiverAddress = asker.address(),
        )
        val askUuid = UUID.randomUUID()
        val createResponse = createAsk(
            createAsk = CreateAsk(
                ask = CoinTradeAsk(
                    id = askUuid.toString(),
                    quote = newCoins(150, "nhash"),
                    base = newCoins(100, askerDenom),
                ),
            )
        )
        assertEquals(
            expected = 0L,
            actual = pbClient.getBalance(asker.address(), askerDenom),
            message = "The base should be withdrawn from the asker's account",
        )
        val cancelResponse = cancelAsk(askUuid.toString())
        assertEquals(
            expected = createResponse.askOrder,
            actual = cancelResponse.cancelledAskOrder,
            message = "The cancelled ask order should be included in the response",
        )
        assertTrue(
            actual = cancelResponse.collateralReleased,
            message = "The collateral should always be released in coin trades",
        )
        assertEquals(
            expected = 100L,
            actual = pbClient.getBalance(asker.address(), askerDenom),
            message = "After cancelling an ask, the base should be returned to the asker",
        )
    }

    @Test
    fun testCancelBid() {
        val bidderDenom = "cointradecancelbid"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(amount = 100, denom = bidderDenom),
            receiverAddress = bidder.address(),
        )
        val bidUuid = UUID.randomUUID()
        val createResponse = createBid(
            createBid = CreateBid(
                bid = CoinTradeBid(
                    id = bidUuid.toString(),
                    quote = newCoins(100, bidderDenom),
                    base = newCoins(150, "nhash"),
                ),
            ),
        )
        assertEquals(
            expected = 0L,
            actual = pbClient.getBalance(bidder.address(), bidderDenom),
            message = "The quote should be withdrawn from the bidder's account",
        )
        val cancelResponse = cancelBid(bidId = bidUuid.toString())
        assertEquals(
            expected = createResponse.bidOrder,
            actual = cancelResponse.cancelledBidOrder,
            message = "The cancelled bid order should be included in the response",
        )
        assertEquals(
            expected = 100L,
            actual = pbClient.getBalance(bidder.address(), bidderDenom),
            message = "After canceling a bid, the quote should be returned to the bidder",
        )
    }

    @Test
    fun testUpdateAsk() {
        val baseDenom = "updatecointradeaskbase"
        val base2Denom = "updatecointradeaskbase2"
        val quoteDenom = "updatecointradeaskquote"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, baseDenom),
            receiverAddress = asker.address(),
        )
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, base2Denom),
            receiverAddress = asker.address(),
        )
        val askUuid = UUID.randomUUID()
        assertSucceeds("Initial ask should be created without error") {
            createAsk(
                createAsk = CreateAsk(
                    ask = CoinTradeAsk(
                        id = askUuid.toString(),
                        base = newCoins(100, baseDenom),
                        quote = newCoins(100, quoteDenom),
                    ),
                ),
            )
        }
        assertEquals(
            expected = 900,
            actual = pbClient.getBalance(asker.address(), baseDenom),
            message = "The asker's account should have been debited of its base denom",
        )
        val updateResponse = assertSucceeds("Update ask should succeed without error") {
            updateAsk(
                updateAsk = UpdateAsk(
                    ask = CoinTradeAsk(
                        id = askUuid.toString(),
                        base = newCoins(600, base2Denom),
                        quote = newCoins(200, quoteDenom),
                    )
                ),
            )
        }
        val updatedCollateral = updateResponse.updatedAskOrder.testGetCoinTrade()
        assertEquals(
            expected = newCoins(600, base2Denom),
            actual = updatedCollateral.base,
            message = "Expected the base to be properly updated",
        )
        assertEquals(
            expected = newCoins(200, quoteDenom),
            actual = updatedCollateral.quote,
            message = "Expected the quote to be properly updated",
        )
        assertEquals(
            expected = 1000,
            actual = pbClient.getBalance(asker.address(), baseDenom),
            message = "The asker's base denom should have been refunded from the initial ask, restoring their balance to max",
        )
        assertEquals(
            expected = 400,
            actual = pbClient.getBalance(asker.address(), base2Denom),
            message = "The asker's account should have been debited of its base2 denom",
        )
        val cancelResponse = cancelAsk(askUuid.toString())
        assertEquals(
            expected = updateResponse.updatedAskOrder,
            actual = cancelResponse.cancelledAskOrder,
            message = "The cancel response should include the cancelled ask order",
        )
        assertEquals(
            expected = 1000,
            actual = pbClient.getBalance(asker.address(), base2Denom),
            message = "After cancelling the updated order, the asker should have all of its base2Denom amount from a refund",
        )
    }

    @Test
    fun testUpdateBidToSameType() {
        val quoteDenom = "updatecointradeaskquote"
        val quote2Denom = "updatecointradeaskquote2"
        val baseDenom = "updatecointradeaskbase"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, quoteDenom),
            receiverAddress = bidder.address(),
        )
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, quote2Denom),
            receiverAddress = bidder.address(),
        )
        val bidUuid = UUID.randomUUID()
        assertSucceeds("Initial ask should be created without error") {
            createBid(
                createBid = CreateBid(
                    bid = CoinTradeBid(
                        id = bidUuid.toString(),
                        quote = newCoins(100, quoteDenom),
                        base = newCoins(100, baseDenom),
                    ),
                ),
            )
        }
        assertEquals(
            expected = 900,
            actual = pbClient.getBalance(bidder.address(), quoteDenom),
            message = "The bidder's account should have been debited of its quote denom",
        )
        val updateResponse = assertSucceeds("Update bid should succeed without error") {
            updateBid(
                updateBid = UpdateBid(
                    bid = CoinTradeBid(
                        id = bidUuid.toString(),
                        quote = newCoins(600, quote2Denom),
                        base = newCoins(200, baseDenom),
                    )
                ),
            )
        }
        val updatedCollateral = updateResponse.updatedBidOrder.testGetCoinTrade()
        assertEquals(
            expected = newCoins(600, quote2Denom),
            actual = updatedCollateral.quote,
            message = "Expected the quote to be properly updated",
        )
        assertEquals(
            expected = newCoins(200, baseDenom),
            actual = updatedCollateral.base,
            message = "Expected the base to be properly updated",
        )
        assertEquals(
            expected = 1000,
            actual = pbClient.getBalance(bidder.address(), quoteDenom),
            message = "The bidder's quote denom should have been refunded from the initial bid, restoring their balance to max",
        )
        assertEquals(
            expected = 400,
            actual = pbClient.getBalance(bidder.address(), quote2Denom),
            message = "The bidder's account should have been debited of its quote2 denom",
        )
        val cancelResponse = cancelBid(bidUuid.toString(), bidder)
        assertEquals(
            expected = updateResponse.updatedBidOrder,
            actual = cancelResponse.cancelledBidOrder,
            message = "The cancel response should include the cancelled bid order",
        )
        assertEquals(
            expected = 1000,
            actual = pbClient.getBalance(bidder.address(), quote2Denom),
            message = "After cancelling the updated order, the bidder should have all of its quote2Denom amount from a refund",
        )
    }

    @Test
    fun testUpdateBidToNewType() {
        val quoteDenom = "updatecointradenewtypeaskquote"
        val quote2Denom = "updatecointradenewtypeaskquote2"
        val baseDenom = "updatecointradenewtypeaskbase"
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, quoteDenom),
            receiverAddress = bidder.address(),
        )
        giveTestDenom(
            pbClient = pbClient,
            initialHoldings = newCoin(1000, quote2Denom),
            receiverAddress = bidder.address(),
        )
        val bidUuid = UUID.randomUUID()
        assertSucceeds("Initial ask should be created without error") {
            createBid(
                createBid = CreateBid(
                    bid = CoinTradeBid(
                        id = bidUuid.toString(),
                        quote = newCoins(100, quoteDenom),
                        base = newCoins(100, baseDenom),
                    ),
                ),
            )
        }
        assertEquals(
            expected = 900,
            actual = pbClient.getBalance(bidder.address(), quoteDenom),
            message = "The bidder's account should have been debited of its quote denom",
        )
        val updateResponse = assertSucceeds("Update bid should succeed without error") {
            updateBid(
                updateBid = UpdateBid(
                    bid = ScopeTradeBid(
                        id = bidUuid.toString(),
                        quote = newCoins(600, quote2Denom),
                        scopeAddress = "some rando scope",
                    )
                ),
            )
        }
        val updatedCollateral = updateResponse.updatedBidOrder.testGetScopeTrade()
        assertEquals(
            expected = newCoins(600, quote2Denom),
            actual = updatedCollateral.quote,
            message = "Expected the quote to be properly updated",
        )
        assertEquals(
            expected = "some rando scope",
            actual = updatedCollateral.scopeAddress,
            message = "Expected the scope address to be properly updated",
        )
        assertEquals(
            expected = 1000,
            actual = pbClient.getBalance(bidder.address(), quoteDenom),
            message = "The bidder's quote denom should have been refunded from the initial bid, restoring their balance to max",
        )
        assertEquals(
            expected = 400,
            actual = pbClient.getBalance(bidder.address(), quote2Denom),
            message = "The bidder's account should have been debited of its quote2 denom",
        )
        val cancelResponse = cancelBid(bidUuid.toString(), bidder)
        assertEquals(
            expected = updateResponse.updatedBidOrder,
            actual = cancelResponse.cancelledBidOrder,
            message = "The cancel response should include the cancelled bid order",
        )
        assertEquals(
            expected = 1000,
            actual = pbClient.getBalance(bidder.address(), quote2Denom),
            message = "After cancelling the updated order, the bidder should have all of its quote2Denom amount from a refund",
        )
    }

    @Test
    fun testGetAsksByCollateralId() {
        val askUuid = UUID.randomUUID()
        val response = createAsk(
            createAsk = CreateAsk(
                ask = CoinTradeAsk(
                    id = askUuid.toString(),
                    quote = newCoins(150, "nhash"),
                    base = newCoins(100, "nhash"),
                )
            )
        )
        // Create another ask to ensure multiple asks CAN be returned but aren't
        createAsk(
            createAsk = CreateAsk(
                ask = CoinTradeAsk(
                    id = UUID.randomUUID().toString(),
                    quote = newCoins(150, "nhash"),
                    base = newCoins(100, "nhash"),
                )
            )
        )
        val queryAskOrder = bilateralClient.getAsksByCollateralId(askUuid.toString()).assertSingle("One ask should be returned by the collateral id query")
        assertEquals(
            expected = response.askOrder,
            actual = queryAskOrder,
            message = "The correct ask should be returned in the collateral id query",
        )
    }
}
