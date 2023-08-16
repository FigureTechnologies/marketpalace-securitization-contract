package io.provenance.bilateral.exceptions

/**
 * This exception is thrown when the Metadata Bilateral Exchange smart contract returns a null data payload when a query
 * is made through a function with a non-null response type.
 */
class NullContractResultException(message: String, e: Exception? = null) : Exception(message, e)
