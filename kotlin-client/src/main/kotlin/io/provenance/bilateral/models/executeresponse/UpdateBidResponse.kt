package io.provenance.bilateral.models.executeresponse

import io.provenance.bilateral.models.BidOrder

/**
 * The data response returned after a bid is successfully updated.
 *
 * @param bidId The unique identifier of the updated bid.
 * @param updatedBidOrder The [BidOrder]'s structure after the update has been executed.  This value directly
 * represents the current state of the order in the contract.
 */
data class UpdateBidResponse(val bidId: String, val updatedBidOrder: BidOrder)
