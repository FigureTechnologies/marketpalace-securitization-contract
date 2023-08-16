package io.provenance.bilateral.models.enums

import com.fasterxml.jackson.annotation.JsonProperty

/**
 * Correlates to a type of ask or bid order, for quick identification purposes.  This value will always match the
 * collateral set on an [io.provenance.bilateral.models.AskOrder] or [io.provenance.bilateral.models.BidOrder].
 */
enum class BilateralRequestType {
    @JsonProperty("coin_trade") COIN_TRADE,
    @JsonProperty("marker_trade") MARKER_TRADE,
    @JsonProperty("marker_share_sale") MARKER_SHARE_SALE,
    @JsonProperty("scope_trade") SCOPE_TRADE,
}
