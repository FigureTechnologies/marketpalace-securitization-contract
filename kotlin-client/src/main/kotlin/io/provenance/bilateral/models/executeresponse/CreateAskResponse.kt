package io.provenance.bilateral.models.executeresponse

import io.provenance.bilateral.models.AskOrder

/**
 * The data response returned after an ask is successfully created.
 *
 * @param askId The unique identifier of the created ask.
 * @param askFeeCharged A human-readable display of how much nhash was charged as a fee.  If no fee is configured
 * (the fee will be set as zero nhash in the [io.provenance.bilateral.models.ContractInfo.createAskNhashFee] value) then
 * this value will be null.
 * @param askOrder The [AskOrder] created by the request.  This value directly represents the current state of the order
 * in the contract.
 */
data class CreateAskResponse(
    val askId: String,
    val askFeeCharged: String?,
    val askOrder: AskOrder,
)
