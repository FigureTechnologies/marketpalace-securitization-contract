package io.provenance.bilateral.models.executeresponse

import io.provenance.bilateral.models.BidOrder

/**
 * The data response returned after a successful bid cancellation.
 *
 * @param bidId The unique identifier of the cancelled bid.
 * @param cancelledBidOrder The [BidOrder] that was held by the contract prior to the bid's cancellation.  This is the
 * final remnant of the bid, because on a successful cancellation, the [BidOrder] is fully deleted from the contract.
 */
data class CancelBidResponse(val bidId: String, val cancelledBidOrder: BidOrder)
