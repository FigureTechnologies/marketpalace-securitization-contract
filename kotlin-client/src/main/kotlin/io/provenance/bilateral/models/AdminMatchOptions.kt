package io.provenance.bilateral.models

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming

/**
 * Variants of the admin match options for each different request type.  Each type includes its own subset of configurations
 * that may be subject to change as contract features are added.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
sealed interface AdminMatchOptions {
    /**
     * Admin options available when executing a match for two coin trades.  Requests including coin trade admin options
     * for non-coin-trade matches will be rejected.
     *
     * @param acceptMismatchedBids If true, a match will be executed even if the bid offers a lower (or even completely
     * different denom) coin than was requested in the ask's quote.  Ex: Asker requests 200nhash and bidder offers
     * 100nhash - refused unless this flag is 'true'.  Ex: Asker requests 200nhash and bidder offers 500000dogecoin -
     * refused unless this flag is 'true'.
     */
    @JsonNaming(SnakeCaseStrategy::class)
    @JsonTypeName("coin_trade")
    data class CoinTradeAdminOptions(
        val acceptMismatchedBids: Boolean? = null,
    ) : AdminMatchOptions {
        override fun toString(): String = "coin_trade: accept_mismatched_bids = $acceptMismatchedBids"
    }

    /**
     * Admin options available when executing a match for two marker trades.  Requests including marker trade admin
     * options for non-marker-trade matches will be rejected.
     *
     * @param acceptMismatchedBids If true, a match will be executed even if the bid offers a lower (or even completely
     * different denom) coin than was requested in the ask's quote.  Ex: Asker requests 200nhash and bidder offers
     * 100nhash - refused unless this flag is 'true'.  Ex: Asker requests 200nhash and bidder offers 500000dogecoin -
     * refused unless this flag is 'true'.
     */
    @JsonNaming(SnakeCaseStrategy::class)
    @JsonTypeName("marker_trade")
    data class MarkerTradeAdminOptions(
        val acceptMismatchedBids: Boolean? = null,
    ) : AdminMatchOptions {
        override fun toString(): String = "marker_trade: accept_mismatched_bids = $acceptMismatchedBids"
    }

    /**
     * Admin options available when executing a match for two marker share sales.  Requests including marker share sale
     * admin options for non-marker-share-sale matches will be rejected.
     *
     * @param overrideQuoteSource If provided, the target's quote will be used to determine how much quote funds are
     * sent to the asker after the trade completes.  The contract will automatically calculate a quote per share value
     * for the ask or bid based on values stored with the ask or bid order, and use that value alongside the bid's
     * amount of shares being purchased to determine how much funds will be sent to the asker.
     */
    @JsonNaming(SnakeCaseStrategy::class)
    @JsonTypeName("marker_share_sale")
    data class MarkerShareSaleAdminOptions(
        val overrideQuoteSource: OverrideQuoteSource? = null,
    ) : AdminMatchOptions {
        override fun toString(): String = "marker_share_sale: override_quote_source = $overrideQuoteSource"
    }

    /**
     * Admin options available when executing a match for two scope trades.  Requests including scope trade admin
     * options for non-scope-trade matches will be rejected.
     *
     * @param acceptMismatchedBids If true, a match will be executed even if the bid offers a lower (or even completely
     * different denom) coin than was requested in the ask's quote.  Ex: Asker requests 200nhash and bidder offers
     * 100nhash - refused unless this flag is 'true'.  Ex: Asker requests 200nhash and bidder offers 500000dogecoin -
     * refused unless this flag is 'true'.
     */
    @JsonNaming(SnakeCaseStrategy::class)
    @JsonTypeName("scope_trade")
    data class ScopeTradeAdminOptions(
        val acceptMismatchedBids: Boolean? = null,
    ) : AdminMatchOptions {
        override fun toString(): String = "scope_trade: accept_mismatched_bids = $acceptMismatchedBids"
    }
}

enum class OverrideQuoteSource {
    @JsonProperty("ask")
    ASK,
    @JsonProperty("bid")
    BID,
}
