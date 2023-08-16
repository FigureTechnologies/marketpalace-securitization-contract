package io.provenance.bilateral.models

import com.fasterxml.jackson.databind.PropertyNamingStrategies
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.serialization.CosmWasmUintToBigIntegerDeserializer
import java.math.BigInteger

/**
 * Contains the results of a search made by using the [io.provenance.bilateral.client.BilateralContractClient.searchAsks]
 * or [io.provenance.bilateral.client.BilateralContractClient.searchBids] functions, or their "OrNull" variants.  Searches
 * are paginated by default.
 *
 * @param results The resulting [io.provenance.bilateral.models.AskOrder] or [io.provenance.bilateral.models.BidOrder]
 * values from the search.  If no results are found, this list will be empty.
 * @param pageNumber The current page being returned by the search.
 * @param pageSize The maximum size of the page being returned by the search.  This value will either match the size of
 * the [results] or be greater than it, indicating that more results were requested than were retrieved.
 * @param totalPages The total number of pages that match the search parameters, given the input [pageSize].
 */
@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class ContractSearchResult<T>(
    val results: List<T>,
    @JsonDeserialize(using = CosmWasmUintToBigIntegerDeserializer::class)
    val pageNumber: BigInteger,
    @JsonDeserialize(using = CosmWasmUintToBigIntegerDeserializer::class)
    val pageSize: BigInteger,
    @JsonDeserialize(using = CosmWasmUintToBigIntegerDeserializer::class)
    val totalPages: BigInteger,
)
