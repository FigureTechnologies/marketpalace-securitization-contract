package io.provenance.bilateral.query

import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.annotation.JsonValue
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import io.provenance.bilateral.interfaces.BilateralContractQueryMsg
import io.provenance.bilateral.models.enums.BilateralRequestType
import io.provenance.bilateral.serialization.CosmWasmBigIntegerToUintSerializer
import java.math.BigInteger

/**
 * The core request structure for searching the contract, using [io.provenance.bilateral.client.BilateralContractClient.searchAsks],
 * [io.provenance.bilateral.client.BilateralContractClient.searchBids], or the "OrNull" variants of those functions.
 * The result produced is paginated.
 *
 * @param searchType Defines the search target for the request.
 * @param pageSize The size of page to use for the search.  If not specified, the [DEFAULT_PAGE_SIZE] value is used.
 * @param pageNumber The page number to request.  If not specified, the [DEFAULT_PAGE_NUMBER] value is used.
 */
@JsonNaming(SnakeCaseStrategy::class)
data class ContractSearchRequest(
    val searchType: ContractSearchType,
    @JsonSerialize(using = CosmWasmBigIntegerToUintSerializer::class)
    val pageSize: BigInteger? = null,
    @JsonSerialize(using = CosmWasmBigIntegerToUintSerializer::class)
    val pageNumber: BigInteger? = null,
) {
    companion object {
        val DEFAULT_PAGE_SIZE: BigInteger = BigInteger.TEN
        val MAX_PAGE_SIZE: BigInteger = 25.toBigInteger()
        val MIN_PAGE_SIZE: BigInteger = BigInteger.ONE
        val DEFAULT_PAGE_NUMBER: BigInteger = BigInteger.ONE
        val MIN_PAGE_NUMBER: BigInteger = BigInteger.ONE
    }

    internal fun searchAsks(): SearchAsks = SearchAsks(this)
    internal fun searchBids(): SearchBids = SearchBids(this)

    @JsonIgnore
    internal fun getLoggingSuffix(): String = when (this.searchType) {
        is ContractSearchType.All -> "[all]"
        is ContractSearchType.Type -> "[type], type = [${this.searchType.valueType}]"
        is ContractSearchType.Id -> "[id], id = [${this.searchType.id}]"
        is ContractSearchType.Owner -> "[owner], owner = [${this.searchType.owner}]"
    }.let { searchTypeString -> "searchType = $searchTypeString, pageSize = [${pageSize ?: "DEFAULT"}], pageNumber = [${pageNumber ?: "DEFAULT"}]" }
}

/**
 * An internal model used to search for [io.provenance.bilateral.models.AskOrder] values.
 *
 * @param search The search parameters for the request.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("search_asks")
data class SearchAsks(val search: ContractSearchRequest) : BilateralContractQueryMsg {
    override fun toLoggingString(): String = "searchAsks, ${search.getLoggingSuffix()}"
}

/**
 * An internal model used to search for [io.provenance.bilateral.models.BidOrder] values.
 *
 * @param search The search parameters for the request.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("search_bids")
data class SearchBids(val search: ContractSearchRequest) : BilateralContractQueryMsg {
    override fun toLoggingString(): String = "searchBids, ${search.getLoggingSuffix()}"
}

/**
 * The search parameters for a search request.  Each type indicates a different target value by which to locate ask or
 * bid orders.
 */
sealed interface ContractSearchType {
    /**
     * Simply retrieves all stored ask or bid orders without any specialized target values.
     */
    @JsonNaming(SnakeCaseStrategy::class)
    object All : ContractSearchType {
        @JsonValue
        fun serializeAs(): String = "all"
    }

    /**
     * Retrieves all ask or bid orders that match a specific [BilateralRequestType].
     */
    @JsonNaming(SnakeCaseStrategy::class)
    class Type private constructor(val valueType: Body) : ContractSearchType {
        constructor(valueType: BilateralRequestType) : this(Body(valueType))

        @JsonNaming(SnakeCaseStrategy::class)
        class Body(val valueType: BilateralRequestType)
    }

    /**
     * Retrieves all ask or bid orders that match a specific ask or bid id.  Ask and bid ids are unique, so this should
     * only ever produce a single result.
     */
    @JsonNaming(SnakeCaseStrategy::class)
    class Id private constructor(val id: Body) : ContractSearchType {
        constructor(id: String) : this(Body(id))

        @JsonNaming(SnakeCaseStrategy::class)
        class Body(val id: String)
    }

    /**
     * Retrieves all ask or bid orders that match a specific bech32 address owner.
     */
    @JsonNaming(SnakeCaseStrategy::class)
    class Owner private constructor(val owner: Body) : ContractSearchType {
        constructor(owner: String) : this(Body(owner))

        @JsonNaming(SnakeCaseStrategy::class)
        class Body(val owner: String)
    }
}
