package io.provenance.bilateral.models.executeresponse

import io.provenance.bilateral.models.BidOrder

/**
 * The data response returned after a bid is successfully created.
 *
 * @param bidId The unique identifier of the created bid.
 * @param bidFeeCharged A human-readable display of how much nhash was charged as a fee.  If no fee is configured
 * (the fee will be set as zero nhash in the [io.provenance.bilateral.models.ContractInfo.createBidNhashFee] value) then
 * this value will be null.
 * @param bidOrder The [BidOrder] created by the request.  This value directly represents the current state of the order
 * in the contract.
 */
data class CreateBidResponse(
    val bidId: String,
    val bidFeeCharged: String?,
    val bidOrder: BidOrder,
)
