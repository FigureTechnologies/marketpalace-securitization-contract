package io.provenance.bilateral.contract

import io.provenance.bilateral.execute.Ask.CoinTradeAsk
import io.provenance.bilateral.execute.Bid.CoinTradeBid
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import org.junit.jupiter.api.Test
import testconfiguration.ContractIntTest
import testconfiguration.functions.assertNotNull
import testconfiguration.functions.assertNull
import testconfiguration.functions.newCoins
import java.util.UUID
import kotlin.test.assertFails
import kotlin.test.assertTrue

class QueryIntTest : ContractIntTest() {
    @Test
    fun testGetAskFunctions() {
        assertFails("A missing id should cause an exception for getAsk") { bilateralClient.getAsk("some fake ask") }
        bilateralClient.getAskOrNull("some id or something").assertNull("The OrNull variant of getAsk should return null for a missing id")
        val coinTradeAskUuid = UUID.randomUUID()
        val createAsk = CreateAsk(
            ask = CoinTradeAsk(
                id = coinTradeAskUuid.toString(),
                quote = newCoins(100, "nhash"),
                base = newCoins(150, "nhash"),
            ),
        )
        createAsk(createAsk)
        bilateralClient.getAskOrNull(createAsk.ask.mapToId()).assertNotNull("Ask should exist when fetched by nullable request")
    }

    @Test
    fun testGetBidFunctions() {
        assertFails("A missing id should cause an exception for getBid") { bilateralClient.getBid("some fake bid") }
        bilateralClient.getBidOrNull("some id or whatever").assertNull("The OrNull variant of getBid should return null for a missing id")
        val coinTradeBidUuid = UUID.randomUUID()
        val createBid = CreateBid(
            bid = CoinTradeBid(
                id = coinTradeBidUuid.toString(),
                quote = newCoins(100, "nhash"),
                base = newCoins(150, "nhash"),
            ),
        )
        createBid(createBid)
        bilateralClient.getBidOrNull(createBid.bid.mapToId()).assertNotNull("Bid should exist when fetched by nullable request")
    }

    @Test
    fun testGetAsksByCollateralIdEmptyResults() {
        assertTrue(
            actual = bilateralClient.getAsksByCollateralId("some id").isEmpty(),
            message = "When no value is present, the result should be empty for getAsksByCollateralId",
        )
        assertTrue(
            actual = bilateralClient.getAsksByCollateralIdOrNull("some id")?.isEmpty() == true,
            message = "When no value is present, the result should be empty for getAsksByCollateralIdOrNull",
        )
    }
}
