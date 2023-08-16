package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies
import com.fasterxml.jackson.databind.annotation.JsonNaming
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import cosmos.base.v1beta1.CoinOuterClass.Coin
import io.provenance.bilateral.serialization.CosmWasmBigIntegerToUintSerializer
import java.math.BigInteger

/**
 * The structure required to create a Bid within the Metadata Bilateral Exchange smart contract.  Each implementation
 * represents a different bid type.
 */
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
sealed interface Bid {
    /**
     * A bid that specifies coin as the collateral [quote] and requests to match with an ask with the listed [base].
     *
     * @param id The unique identifier for the bid.
     * @param quote The amount of funds sent by the bidder.  This amount will be transferred to the asker on a
     * successful match.
     * @param base The amount of funds to be received by the bidder on a successful match.
     */
    @JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
    @JsonTypeName("coin_trade")
    data class CoinTradeBid(
        val id: String,
        // The quote is used for funds, and never added to the json payload send to the contract
        @JsonIgnore
        val quote: List<Coin>,
        val base: List<Coin>,
    ) : Bid

    /**
     * A bid that specifies coin as the collateral [quote] and requests to receive a marker in exchange.
     *
     * @param id The unique identifier for the bid.
     * @param markerDenom The denomination of the marker that the bid requests in exchange for the [quote].
     * @param withdrawSharesAfterMatch If set to true, all shares of the marker's denom will be withdrawn and sent to
     * the bidder after a successful match is made.
     * @param quote The amount of funds sent by the bidder.  This amount will be transferred to the asker on a successful
     * match.
     */
    @JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
    @JsonTypeName("marker_trade")
    data class MarkerTradeBid(
        val id: String,
        val markerDenom: String,
        val withdrawSharesAfterMatch: Boolean? = null,
        // The quote is used for funds, and never added to the json payload send to the contract
        @JsonIgnore
        val quote: List<Coin>,
    ) : Bid

    /**
     * A bid that specifies coin as the collateral [quote] and requests to receive a specific number of marker shares
     * in exchange.
     *
     * @param id The unique identifier for the bid.
     * @param markerDenom The denomination of the marker shares that the bid requests in exchange for the quote.
     * @param shareCount The amount of marker shares to purchase.  Ex: If the [markerDenom] is "example" and the
     * [shareCount] is 50, the bidder will receive 50example after the match completes.
     * @param quote The amount of funds sent by the bidder.  This amount will be transferred to the asker on a successful
     * match.
     */
    @JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
    @JsonTypeName("marker_share_sale")
    data class MarkerShareSaleBid(
        val id: String,
        val markerDenom: String,
        @JsonSerialize(using = CosmWasmBigIntegerToUintSerializer::class)
        val shareCount: BigInteger,
        // The quote is used for funds, and never added to the json payload send to the contract
        @JsonIgnore
        val quote: List<Coin>,
    ) : Bid

    /**
     * A bid that specifies coin as the collateral [quote] and requests to receive a scope in exchange.
     *
     * @param id The unique identifier for the bid.
     * @param scopeAddress The bech32 address for the scope to be received by the bidder.
     * @param quote The amount of funds sent by the bidder.  This amount will be transferred to the asker on a successful
     * match.
     */
    @JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
    @JsonTypeName("scope_trade")
    data class ScopeTradeBid(
        val id: String,
        val scopeAddress: String,
        // The quote is used for funds, and never added to the json payload send to the contract
        @JsonIgnore
        val quote: List<Coin>,
    ) : Bid

    /**
     * Allows the bid type to be consumed and mapped based on value.  This can be used to derive an output type for any
     * of the request types.
     */
    fun <T> map(
        coinTrade: (coinTrade: CoinTradeBid) -> T,
        markerTrade: (markerTrade: MarkerTradeBid) -> T,
        markerShareSale: (markerShareSale: MarkerShareSaleBid) -> T,
        scopeTrade: (scopeTrade: ScopeTradeBid) -> T,
    ): T = when (this) {
        is CoinTradeBid -> coinTrade(this)
        is MarkerTradeBid -> markerTrade(this)
        is MarkerShareSaleBid -> markerShareSale(this)
        is ScopeTradeBid -> scopeTrade(this)
    }

    /**
     * Maps each type of bid to its respective id property.
     */
    fun mapToId(): String = map(
        coinTrade = { coinTrade -> coinTrade.id },
        markerTrade = { markerTrade -> markerTrade.id },
        markerShareSale = { markerShareSale -> markerShareSale.id },
        scopeTrade = { scopeTrade -> scopeTrade.id },
    )

    /**
     * Maps each type of bid to its funds that will be required during its creation.  This function is used to establish
     * the correct amount of funds that must be sent by the bidder when creating a bid.
     */
    fun mapToFunds(): List<Coin> = map(
        coinTrade = { coinTrade -> coinTrade.quote },
        markerTrade = { markerTrade -> markerTrade.quote },
        markerShareSale = { markerShareSale -> markerShareSale.quote },
        scopeTrade = { scopeTrade -> scopeTrade.quote },
    )
}

/**
 * This is declared as an internal extension function to be used with both the CreateBid and UpdateBid toLoggingString
 * function overrides.  Interfaces do not allow internal function declarations, so this allows this functionality to
 * be private to the library without exposing unnecessary details to consumers.
 */
internal fun Bid.toLoggingStringSuffix(): String = this.map(
    coinTrade = { "bidType = [coin_trade], id = [${it.id}]" },
    markerTrade = { "bidType = [marker_trade], id = [${it.id}], markerDenom = [${it.markerDenom}]" },
    markerShareSale = { "bidType = [marker_share_sale], id = [${it.id}], markerDenom = [${it.markerDenom}], shareCount = [${it.shareCount}]" },
    scopeTrade = { "bidType = [scope_trade], id = [${it.id}], scopeAddress = [${it.scopeAddress}]" },
)
