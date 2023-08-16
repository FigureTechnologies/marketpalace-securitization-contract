package io.provenance.bilateral.contract

import io.provenance.bilateral.execute.Ask.CoinTradeAsk
import io.provenance.bilateral.execute.Bid.CoinTradeBid
import io.provenance.bilateral.execute.CreateAsk
import io.provenance.bilateral.execute.CreateBid
import io.provenance.bilateral.models.AdminMatchOptions.CoinTradeAdminOptions
import org.junit.jupiter.api.Test
import testconfiguration.ContractIntTest
import testconfiguration.functions.newCoins
import java.util.UUID
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

class MatchReportIntTest : ContractIntTest() {
    @Test
    fun testSimpleMatchReport() {
        val askId = UUID.randomUUID().toString()
        createAsk(
            createAsk = CreateAsk(
                ask = CoinTradeAsk(
                    id = askId,
                    quote = newCoins(100, "nhash"),
                    base = newCoins(100, "nhash"),
                ),
            ),
        )
        val bidId = UUID.randomUUID().toString()
        createBid(
            createBid = CreateBid(
                bid = CoinTradeBid(
                    id = bidId,
                    quote = newCoins(100, "nhash"),
                    base = newCoins(100, "nhash"),
                ),
            ),
        )
        val matchReport = bilateralClient.getMatchReport(askId, bidId)
        assertEquals(
            expected = askId,
            actual = matchReport.askId,
            message = "The ask ids should match",
        )
        assertEquals(
            expected = bidId,
            actual = matchReport.bidId,
            message = "The bid ids should match",
        )
        assertTrue(
            actual = matchReport.askExists,
            message = "The ask should be marked as existing",
        )
        assertTrue(
            actual = matchReport.bidExists,
            message = "The bid should be marked as existing",
        )
        assertTrue(
            actual = matchReport.matchPossible,
            message = "The report should indicate that a match is possible",
        )
        assertNull(
            actual = matchReport.errorMessage,
            message = "The report should not include an error message, but was: ${matchReport.errorMessage}",
        )
    }

    @Test
    fun testMatchReportWithAdminOptions() {
        val askId = UUID.randomUUID().toString()
        createAsk(
            createAsk = CreateAsk(
                ask = CoinTradeAsk(
                    id = askId,
                    quote = newCoins(100, "nhash"),
                    base = newCoins(100, "nhash"),
                )
            )
        )
        val bidId = UUID.randomUUID().toString()
        createBid(
            createBid = CreateBid(
                bid = CoinTradeBid(
                    id = bidId,
                    quote = newCoins(50, "nhash"),
                    base = newCoins(100, "nhash"),
                )
            )
        )
        val matchReportWithoutOptions = bilateralClient.getMatchReport(askId, bidId)
        assertFalse(
            actual = matchReportWithoutOptions.matchPossible,
            message = "The match should not be possible without the mismatch bids flag",
        )
        val matchReportWithOptions = bilateralClient.getMatchReport(askId, bidId, CoinTradeAdminOptions(acceptMismatchedBids = true))
        assertTrue(
            actual = matchReportWithOptions.matchPossible,
            message = "The match should be possible with the mismatch bids flag",
        )
    }
}
