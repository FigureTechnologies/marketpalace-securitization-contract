package testconfiguration.functions

import io.provenance.bilateral.client.BilateralContractClient
import io.provenance.bilateral.models.AskOrder
import io.provenance.bilateral.models.BidOrder
import kotlin.test.DefaultAsserter.fail

fun <T> assertSucceeds(message: String = "Expected function invocation to succeed", fn: () -> T): T =
    try {
        fn()
    } catch (e: Exception) {
        fail(message, e)
    }

fun BilateralContractClient.assertAskExists(
    askId: String,
    message: String = "Expected ask [$askId] to exist",
): AskOrder = assertSucceeds(message) { this.getAsk(askId) }

fun BilateralContractClient.assertBidExists(
    bidId: String,
    message: String = "Expected bid [$bidId] to exist",
): BidOrder = assertSucceeds(message) { this.getBid(bidId) }

fun BilateralContractClient.assertAskIsDeleted(askId: String, message: String = "Expected ask [$askId] to not be found, but it existed in the contract") {
    try {
        this.getAsk(askId)
        fail(message)
    } catch (_: Exception) {
        // Success - we want an exception to be thrown, indicating the ask is not present
    }
}

fun BilateralContractClient.assertBidIsDeleted(bidId: String, message: String = "Expected bid [$bidId] to not be found, but it existed in the contract") {
    try {
        this.getBid(bidId)
        fail(message)
    } catch (_: Exception) {
        // Success - we want an exception to be thrown, indicating the bid is not present
    }
}

fun <T> T?.assertNotNull(message: String = "Expected value to not be null"): T {
    kotlin.test.assertNotNull(
        actual = this,
        message = message,
    )
    return this
}

fun <T> T?.assertNull(message: String = "Expected value to be null") {
    kotlin.test.assertNull(
        actual = this,
        message = message,
    )
}

fun <T> Collection<T>.assertSingle(message: String = "Expected a single value to exist within the collection"): T {
    val value = this.singleOrNull()
    kotlin.test.assertNotNull(
        actual = value,
        message = message,
    )
    return value
}

fun <T> Collection<T>.assertSingle(
    message: String = "Expected a single value to match the predicate",
    predicate: (T) -> Boolean,
): T = this.filter(predicate).let { filteredCollection ->
    filteredCollection.assertSingle(message = "$message. Actual amount: ${filteredCollection.size}")
}
