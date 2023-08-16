package io.provenance.bilateral.client

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class BilateralContractClientLoggerTest {
    @Test
    fun testCustomLogger() {
        val infoMessages = mutableListOf<String>()
        val errorMessages = mutableListOf<Pair<String, Exception?>>()
        val logger = BilateralContractClientLogger.Custom(
            infoLogger = { message -> infoMessages.add(message) },
            errorLogger = { message, e -> errorMessages.add(message to e) },
        )
        logger.info("info message")
        assertEquals(
            expected = "info message",
            actual = infoMessages.singleOrNull(),
            message = "A single info message should be sent with the correct text",
        )
        assertTrue(
            actual = errorMessages.isEmpty(),
            message = "No error messages should have been sent yet",
        )
        val exception = Exception("Some exception")
        logger.error("error message", exception)
        assertEquals(
            expected = 1,
            actual = infoMessages.size,
            message = "No extra info messages should be sent",
        )
        assertEquals(
            expected = "error message" to exception,
            actual = errorMessages.singleOrNull(),
            message = "A single error message should be sent with the proper exception",
        )
    }
}
