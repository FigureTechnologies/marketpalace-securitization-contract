package io.provenance.bilateral.models.executeresponse

import io.provenance.bilateral.models.AskOrder

/**
 * The data response returned after an ask is successfully updated.
 *
 * @param askId The unique identifier of the updated ask.
 * @param updatedAskOrder The [AskOrder]'s structure after the update has been executed.  This value directly
 * represents the current state of the order in the contract.
 */
data class UpdateAskResponse(val askId: String, val updatedAskOrder: AskOrder)
