package io.provenance.bilateral.models

import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.fasterxml.jackson.databind.annotation.JsonNaming
import cosmos.base.v1beta1.CoinOuterClass.Coin
import io.provenance.bilateral.models.BidCollateral.CoinTrade
import io.provenance.bilateral.models.BidCollateral.MarkerShareSale
import io.provenance.bilateral.models.BidCollateral.MarkerTrade
import io.provenance.bilateral.models.BidCollateral.ScopeTrade
import io.provenance.bilateral.models.enums.BilateralRequestType
import io.provenance.bilateral.serialization.CosmWasmUintToBigIntegerDeserializer
import java.math.BigInteger

@JsonNaming(SnakeCaseStrategy::class)
data class BidOrder(
    val id: String,
    val bidType: BilateralRequestType,
    val owner: String,
    val collateral: BidCollateral,
    val descriptor: RequestDescriptor?,
) {
    @JsonIgnore
    fun <T> mapCollateral(
        coinTrade: (coinTrade: CoinTrade) -> T,
        markerTrade: (markerTrade: MarkerTrade) -> T,
        markerShareSale: (markerShareSale: MarkerShareSale) -> T,
        scopeTrade: (ScopeTrade) -> T,
    ): T = when (collateral) {
        is CoinTrade -> coinTrade(collateral)
        is MarkerTrade -> markerTrade(collateral)
        is MarkerShareSale -> markerShareSale(collateral)
        is ScopeTrade -> scopeTrade(collateral)
    }
}

@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
sealed interface BidCollateral {
    @JsonNaming(SnakeCaseStrategy::class)
    @JsonTypeName("coin_trade")
    data class CoinTrade(
        val base: List<Coin>,
        val quote: List<Coin>,
    ) : BidCollateral

    @JsonNaming(SnakeCaseStrategy::class)
    @JsonTypeName("marker_trade")
    data class MarkerTrade(
        val markerAddress: String,
        val markerDenom: String,
        val quote: List<Coin>,
        val withdrawSharesAfterMatch: Boolean?,
    ) : BidCollateral

    @JsonNaming(SnakeCaseStrategy::class)
    @JsonTypeName("marker_share_sale")
    data class MarkerShareSale(
        val markerAddress: String,
        val markerDenom: String,
        @JsonDeserialize(using = CosmWasmUintToBigIntegerDeserializer::class)
        val shareCount: BigInteger,
        val quote: List<Coin>,
    ) : BidCollateral

    @JsonNaming(SnakeCaseStrategy::class)
    @JsonTypeName("scope_trade")
    data class ScopeTrade(
        val scopeAddress: String,
        val quote: List<Coin>,
    ) : BidCollateral
}
