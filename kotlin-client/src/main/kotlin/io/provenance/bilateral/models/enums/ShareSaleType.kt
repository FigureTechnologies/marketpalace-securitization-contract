package io.provenance.bilateral.models.enums

import com.fasterxml.jackson.annotation.JsonProperty

/**
 * Specifies the type of marker share sale that is to be executed when a match occurs on the corresponding ask order.
 */
enum class ShareSaleType {
    @JsonProperty("single_transaction") SINGLE_TRANSACTION,
    @JsonProperty("multiple_transactions") MULTIPLE_TRANSACTIONS,
}
