package io.provenance.bilateral.contract

import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode
import io.provenance.bilateral.execute.Ask.CoinTradeAsk
import io.provenance.bilateral.execute.Bid.CoinTradeBid
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import io.provenance.bilateral.models.AttributeRequirement
import io.provenance.bilateral.models.RequestDescriptor
import io.provenance.bilateral.models.enums.AttributeRequirementType
import io.provenance.bilateral.models.enums.BilateralRequestType
import io.provenance.bilateral.query.ContractSearchRequest
import io.provenance.bilateral.query.ContractSearchType
import io.provenance.client.grpc.BaseReqSigner
import io.provenance.client.protobuf.extensions.toAny
import io.provenance.client.protobuf.extensions.toTxBody
import io.provenance.scope.util.toUuid
import org.junit.jupiter.api.Test
import testconfiguration.ContractIntTest
import testconfiguration.extensions.checkIsSuccess
import testconfiguration.functions.assertSingle
import testconfiguration.functions.newCoins
import java.math.BigInteger
import java.time.OffsetDateTime
import java.util.UUID
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class SearchIntTest : ContractIntTest() {
    @Test
    fun testAllSearch() {
        val askUuids = mutableListOf<UUID>()
        val msgs = (0..9).map {
            val askUuid = UUID.randomUUID()
            askUuids += askUuid
            val createAsk = CreateAsk(
                ask = CoinTradeAsk(
                    id = askUuid.toString(),
                    quote = newCoins(100, "nhash"),
                    base = newCoins(100, "nhash"),
                ),
                descriptor = RequestDescriptor(
                    description = "Description",
                    effectiveTime = OffsetDateTime.now(),
                    attributeRequirement = AttributeRequirement(
                        attributes = listOf("a.pb", "b.pb"),
                        requirementType = AttributeRequirementType.ALL,
                    )
                ),
            )
            bilateralClient.generateCreateAskMsg(createAsk, asker.address())
        }
        pbClient.estimateAndBroadcastTx(
            txBody = msgs.map { it.toAny() }.toTxBody(),
            signers = listOf(BaseReqSigner(asker)),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
            gasAdjustment = 1.2,
        ).checkIsSuccess()
        val searchResult = bilateralClient.searchAsks(
            ContractSearchRequest(
                searchType = ContractSearchType.All,
                pageSize = 11.toBigInteger(),
            )
        )
        assertEquals(
            expected = 10,
            actual = searchResult.results.size,
            message = "Expected all results to be returned",
        )
        assertTrue(
            actual = searchResult.results.map { it.id.toUuid() }.all { askUuid -> askUuid in askUuids },
            message = "All ask uuids should be present in the search result",
        )
        assertEquals(
            expected = BigInteger.ONE,
            actual = searchResult.pageNumber,
            message = "The search result should indicate the first page",
        )
        assertEquals(
            expected = BigInteger.ONE,
            actual = searchResult.totalPages,
            message = "The search result should indicate that there is only one total page",
        )
        assertEquals(
            expected = 11.toBigInteger(),
            actual = searchResult.pageSize,
            message = "The page size of the search result should reflect the input",
        )
        // Clean up outstanding asks
        pbClient.estimateAndBroadcastTx(
            txBody = askUuids.map { askUuid ->
                bilateralClient.generateCancelAskMsg(
                    askId = askUuid.toString(),
                    senderAddress = asker.address(),
                ).toAny()
            }.toTxBody(),
            signers = BaseReqSigner(asker).let(::listOf),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        ).checkIsSuccess()
    }

    @Test
    fun testOwnerSearch() {
        val askUuids = mutableListOf<UUID>()
        val msgs = (0..9).map {
            val askUuid = UUID.randomUUID()
            askUuids += askUuid
            val createAsk = CreateAsk(
                ask = CoinTradeAsk(
                    id = askUuid.toString(),
                    quote = newCoins(100, "nhash"),
                    base = newCoins(100, "nhash"),
                ),
                descriptor = RequestDescriptor(
                    description = "Description",
                    effectiveTime = OffsetDateTime.now(),
                    attributeRequirement = AttributeRequirement(
                        attributes = listOf("a.pb", "b.pb"),
                        requirementType = AttributeRequirementType.ALL,
                    )
                ),
            )
            bilateralClient.generateCreateAskMsg(createAsk, asker.address())
        }
        pbClient.estimateAndBroadcastTx(
            txBody = msgs.map { it.toAny() }.toTxBody(),
            signers = listOf(BaseReqSigner(asker)),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
            gasAdjustment = 1.2,
        ).checkIsSuccess()
        val searchResult = bilateralClient.searchAsks(
            ContractSearchRequest(
                searchType = ContractSearchType.Owner(asker.address()),
                pageSize = 11.toBigInteger(),
            )
        )
        assertEquals(
            expected = 10,
            actual = searchResult.results.size,
            message = "Expected all results to be returned",
        )
        assertTrue(
            actual = searchResult.results.map { it.id.toUuid() }.all { askUuid -> askUuid in askUuids },
            message = "All ask uuids should be present in the search result",
        )
        assertEquals(
            expected = BigInteger.ONE,
            actual = searchResult.pageNumber,
            message = "The search result should indicate the first page",
        )
        assertEquals(
            expected = BigInteger.ONE,
            actual = searchResult.totalPages,
            message = "The search result should indicate that there is only one total page",
        )
        assertEquals(
            expected = 11.toBigInteger(),
            actual = searchResult.pageSize,
            message = "The page size of the search result should reflect the input",
        )
        // Clean up outstanding asks
        pbClient.estimateAndBroadcastTx(
            txBody = askUuids.map { askUuid ->
                bilateralClient.generateCancelAskMsg(
                    askId = askUuid.toString(),
                    senderAddress = asker.address(),
                ).toAny()
            }.toTxBody(),
            signers = BaseReqSigner(asker).let(::listOf),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        ).checkIsSuccess()
    }

    @Test
    fun testTypeSearch() {
        val bidUuids = mutableListOf<UUID>()
        val msgs = (0..9).map {
            val bidUuid = UUID.randomUUID()
            bidUuids += bidUuid
            val createBid = CreateBid(
                bid = CoinTradeBid(
                    id = bidUuid.toString(),
                    quote = newCoins(100, "nhash"),
                    base = newCoins(100, "nhash"),
                ),
                descriptor = RequestDescriptor(
                    description = "Description",
                    effectiveTime = OffsetDateTime.now(),
                    attributeRequirement = AttributeRequirement(
                        attributes = listOf("a.pb", "b.pb"),
                        requirementType = AttributeRequirementType.NONE,
                    )
                )
            )
            bilateralClient.generateCreateBidMsg(createBid, bidder.address())
        }
        pbClient.estimateAndBroadcastTx(
            txBody = msgs.map { it.toAny() }.toTxBody(),
            signers = listOf(BaseReqSigner(bidder)),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
            gasAdjustment = 1.2,
        ).checkIsSuccess()
        val searchResult = bilateralClient.searchBids(
            ContractSearchRequest(
                searchType = ContractSearchType.Type(BilateralRequestType.COIN_TRADE),
                pageSize = 11.toBigInteger(),
            )
        )
        assertEquals(
            expected = 10,
            actual = searchResult.results.size,
            message = "Expected all results to be returned",
        )
        assertTrue(
            actual = searchResult.results.map { it.id.toUuid() }.all { bidUuid -> bidUuid in bidUuids },
            message = "All bid uuids should be present in the search result",
        )
        assertEquals(
            expected = BigInteger.ONE,
            actual = searchResult.pageNumber,
            message = "The search result should indicate the first page",
        )
        assertEquals(
            expected = BigInteger.ONE,
            actual = searchResult.totalPages,
            message = "The search result should indicate that there is only one total page",
        )
        assertEquals(
            expected = 11.toBigInteger(),
            actual = searchResult.pageSize,
            message = "The page size of the search result should reflect the input",
        )
        // Clean up outstanding bids
        pbClient.estimateAndBroadcastTx(
            txBody = bidUuids.map { bidUuid ->
                bilateralClient.generateCancelBidMsg(
                    bidId = bidUuid.toString(),
                    senderAddress = bidder.address(),
                ).toAny()
            }.toTxBody(),
            signers = BaseReqSigner(bidder).let(::listOf),
            mode = BroadcastMode.BROADCAST_MODE_BLOCK,
        ).checkIsSuccess()
    }

    @Test
    fun testIdSearch() {
        val askUuid = UUID.randomUUID()
        val createResponse = createAsk(
            createAsk = CreateAsk(
                ask = CoinTradeAsk(
                    id = askUuid.toString(),
                    quote = newCoins(100, "nhash"),
                    base = newCoins(150, "nhash"),
                )
            )
        )
        val searchResult = bilateralClient.searchAsks(
            ContractSearchRequest(
                searchType = ContractSearchType.Id(askUuid.toString()),
                pageSize = 15.toBigInteger(),
            )
        )
        val searchAskOrder = searchResult.results.assertSingle("A single result should be returned for an id search that finds its target")
        assertEquals(
            expected = createResponse.askOrder,
            actual = searchAskOrder,
            message = "The correct ask should be returned",
        )
        assertEquals(
            expected = 1,
            actual = searchResult.results.size,
            message = "Expected all results to be returned",
        )
        assertEquals(
            expected = BigInteger.ONE,
            actual = searchResult.pageNumber,
            message = "Expected the requested page number to be returned",
        )
        assertEquals(
            expected = BigInteger.ONE,
            actual = searchResult.totalPages,
            message = "Expected the correct number of pages to be returned",
        )
        assertEquals(
            expected = 15.toBigInteger(),
            actual = searchResult.pageSize,
            message = "Expected the requested page size to be returned",
        )
    }
}
