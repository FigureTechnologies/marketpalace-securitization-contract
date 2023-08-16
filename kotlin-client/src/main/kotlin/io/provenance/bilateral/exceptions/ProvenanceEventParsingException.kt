package io.provenance.bilateral.exceptions

/**
 * This exception is thrown when an unexpected result is returned in the BroadcastTxResponse for an execute command made
 * to the Metadata Bilateral Exchange smart contract.
 */
class ProvenanceEventParsingException(message: String, e: Exception? = null) : Exception(message, e)
