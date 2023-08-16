package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies
import com.fasterxml.jackson.databind.annotation.JsonNaming
import com.fasterxml.jackson.databind.annotation.JsonSerialize
import cosmos.base.v1beta1.CoinOuterClass.Coin
import io.provenance.bilateral.models.enums.ShareSaleType
import io.provenance.bilateral.serialization.CosmWasmBigIntegerToUintSerializer
import java.math.BigInteger

/**
 * The structure required to create an Ask within the Metadata Bilateral Exchange smart contract.  Each implementation
 * represents a different ask type.
 */
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
sealed interface Ask {
    /**
     * An ask that specifies coin as the collateral [base] and requests a specified amount of coin in return [quote].
     *
     * @param id The unique identifier for the ask.
     * @param quote The amount of funds required to execute a match.  These coins will be sent to the asker when a match
     * is completed.
     * @param base The amount of funds offered in the exchange.  These coins will be included as funds in the request
     * and held in the contract's escrow storage until the exchange is completed.  Once completed, they will be
     * transferred directly to the bidder.
     */
    @JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
    @JsonTypeName("coin_trade")
    data class CoinTradeAsk(
        val id: String,
        val quote: List<Coin>,
        // This value is used as funds in the client and never included in the json payload
        @JsonIgnore
        val base: List<Coin>,
    ) : Ask

    /**
     * An ask that specifies a marker as the collateral, and requests a specified amount of coin per share held in the
     * marker in return.  After the exchange executes, the asker is fully removed as an admin of the marker, and the
     * bidder receives the admin rights that the asker had, effectively transferring full marker ownership to the
     * bidder in exchange for the quote coin that the asker receives.
     *
     * Note: A marker trade ask must be made AFTER the contract has been granted admin and withdraw rights to the marker
     * being traded.  Validation will reject a specified marker that is listed by an asker without admin rights on it.
     * It is recommended that the contract be given admin and withdraw rights to the marker in the same transaction that
     * lists the marker to ensure that any failures undo the entirety of the process.
     *
     * @param id The unique identifier for the ask.
     * @param markerDenom The denomination of the marker, allowing the contract to find its details for validation.
     * @param quotePerShare The amount of coin per available share of the marker that the bidder must pay in order for
     * a match to execute and the exchange to be made.
     */
    @JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
    @JsonTypeName("marker_trade")
    data class MarkerTradeAsk(
        val id: String,
        val markerDenom: String,
        val quotePerShare: List<Coin>,
    ) : Ask

    /**
     * An ask that specifies a marker as the collateral, and requests a specified amount of coin per share sold to the
     * bidder.  After the exchange executes, the asker receives an amount of coin equivalent to the number of shares
     * sold to the bidder multiplied by the [quotePerShare], and the bidder receives the number of marker shares that
     * they purchased.
     *
     * Note: A marker share sale ask must be made AFTER the contract has been granted admin and withdraw rights to the
     * marker being traded.  Validation will reject a specified marker that is listed by an asker without admin rights
     * on it.  It is recommended that the contract be given admin and withdraw rights to the marker in the same
     * transaction that lists the marker to ensure that any failures undo the entirety of the process.
     *
     * @param id The unique identifier for the ask.
     * @param markerDenom The denomination of the marker, allowing the contract to find its details for validation.
     * @param sharesToSell The amount of shares of the marker that will be sold to a bidder/bidders.  This amount cannot
     * exceed the amount of marker shares held by the marker itself.
     * @param quotePerShare The amount of coin per available share of the marker that the bidder must pay in order for a
     * match to execute and the exchange to be made.
     * @param shareSaleType The type of sale to be created.  In a single transaction sale, a total amount equating to
     * [sharesToSell] multiplied by [quotePerShare] must be paid by the bidder at once for the sale to execute.  In a
     * multiple transaction sale, the bidder can instead purchase from one to as many as [sharesToSell] in a single bid,
     * paying [quotePerShare] multiplied by the amount of shares requested for purchase.
     */
    @JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
    @JsonTypeName("marker_share_sale")
    data class MarkerShareSaleAsk(
        val id: String,
        val markerDenom: String,
        @JsonSerialize(using = CosmWasmBigIntegerToUintSerializer::class)
        val sharesToSell: BigInteger,
        val quotePerShare: List<Coin>,
        val shareSaleType: ShareSaleType,
    ) : Ask

    /**
     * An ask that specifies a scope as the collateral, and requests a specified amount of coin in exchange.  After the
     * exchange executes, the asker receives the amount of coin equivalent to the [quote] and the bidder is set as the
     * sole owner in the scope's owner array, as well as being set to the value owner of the scope.
     *
     * Note: A scope trade ask must be made AFTER the contract has been listed as the sole owner in the scope's owner
     * array, as well as being listed as the value owner of the scope.  It is recommended that the scope be written with
     * these values in the same transaction that creates the ask to ensure that any validation rejections by the contract
     * do not permanently set the contract as the controller of the scope.
     *
     * @param id The unique identifier for the ask.
     * @param scopeAddress The bech32 address of the scope to list in the ask order.
     * @param quote The amount of coin requested in exchange for the scope's ownership.
     */
    @JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
    @JsonTypeName("scope_trade")
    data class ScopeTradeAsk(val id: String, val scopeAddress: String, val quote: List<Coin>) : Ask

    /**
     * Allows the ask type to be consumed and mapped based on value.  This can be used to derive an output type for any
     * of the request types.
     */
    fun <T> map(
        coinTrade: (coinTrade: CoinTradeAsk) -> T,
        markerTrade: (markerTrade: MarkerTradeAsk) -> T,
        markerShareSale: (markerShareSale: MarkerShareSaleAsk) -> T,
        scopeTrade: (scopeTrade: ScopeTradeAsk) -> T,
    ): T = when (this) {
        is CoinTradeAsk -> coinTrade(this)
        is MarkerTradeAsk -> markerTrade(this)
        is MarkerShareSaleAsk -> markerShareSale(this)
        is ScopeTradeAsk -> scopeTrade(this)
    }

    /**
     * Maps each type of ask to its respective id property.
     */
    fun mapToId(): String = map(
        coinTrade = { coinTrade -> coinTrade.id },
        markerTrade = { markerTrade -> markerTrade.id },
        markerShareSale = { markerShareSale -> markerShareSale.id },
        scopeTrade = { scopeTrade -> scopeTrade.id },
    )

    /**
     * Maps each type of ask to its funds that will be required during its creation.  This function is used to establish
     * the correct amount of funds that must be sent by the asker when creating an ask.
     */
    fun mapToFunds(): List<Coin> = map(
        coinTrade = { coinTrade -> coinTrade.base },
        markerTrade = { emptyList() },
        markerShareSale = { emptyList() },
        scopeTrade = { emptyList() },
    )
}

/**
 * This is declared as an internal extension function to be used with both the CreateAsk and UpdateAsk toLoggingString
 * function overrides.  Interfaces do not allow internal function declarations, so this allows this functionality to
 * be private to the library without exposing unnecessary details to consumers.
 */
internal fun Ask.toLoggingStringSuffix(): String = this.map(
    coinTrade = { "askType = [coin_trade], id = [${it.id}]" },
    markerTrade = { "askType = [marker_trade], id = [${it.id}], markerDenom = [${it.markerDenom}]" },
    markerShareSale = { "askType = [marker_share_sale], id = [${it.id}], markerDenom = [${it.markerDenom}], sharesToSell = [${it.sharesToSell}]" },
    scopeTrade = { "askType = [scope_trade], id = [${it.id}], scopeAddress = [${it.scopeAddress}]" },
)
