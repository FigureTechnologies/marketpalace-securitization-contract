package io.provenance.bilateral.client

/**
 * Provides logging functionality for the [BilateralContractClient].  The default value used by the client is
 * [BilateralContractClientLogger.Off], indicating that no logs will be produced.
 */
sealed interface BilateralContractClientLogger {
    /**
     * Logs an info message, denoting an occurrence of an event or a positive or neutral nature has occurred.
     *
     * @param message A string message describing the nature of an event that has occurred.
     */
    fun info(message: String)

    /**
     * Logs an error message with an optional Exception, indicating that an event of a negative nature has occurred.
     *
     * @param message A message describing the error that occurred.
     * @param e If non-null, the exception that was encountered, which will describe the source of the error.
     */
    fun error(message: String, e: Exception?)

    /**
     * Ensures that no logging will be produced when the [BilateralContractClient] is used.  Exceptions will still be
     * thrown when this value is used.
     */
    object Off : BilateralContractClientLogger {
        override fun info(message: String) {}
        override fun error(message: String, e: Exception?) {}
    }

    /**
     * Prints logs from the [BilateralContractClient] to the std system logging without any special formatting.
     */
    object Println : BilateralContractClientLogger {
        override fun info(message: String) {
            println(message)
        }

        override fun error(message: String, e: Exception?) {
            System.err.println("$message${e?.let { exception -> ": ${exception.message}"} ?: ""}")
        }
    }

    /**
     * Allows for a custom log interceptor to be used.  This can easily be adapted for use with standard logging tools,
     * like Logback.
     */
    class Custom(
        private val infoLogger: (message: String) -> Unit,
        private val errorLogger: (message: String, e: Exception?) -> Unit,
    ) : BilateralContractClientLogger {
        override fun info(message: String) = infoLogger.invoke(message)
        override fun error(message: String, e: Exception?) = errorLogger.invoke(message, e)
    }
}
