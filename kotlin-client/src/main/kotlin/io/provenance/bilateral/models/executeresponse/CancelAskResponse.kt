package io.provenance.bilateral.models.executeresponse

import io.provenance.bilateral.models.AskOrder

/**
 * The data response returned after a successful ask cancellation.
 *
 * @param askId The unique identifier of the cancelled ask.
 * @param collateralReleased Whether or not all collateral held on behalf of the asker within the contract has been
 * returned after cancelling the ask.  This value can be false when multiple marker share sales have been created for
 * a marker, and an ask for the given marker still remains in contract storage after the cancellation of the target ask.
 * @param cancelledAskOrder The [AskOrder] that was held by the contract prior to the ask's cancellation. This is the
 * final remnant of the ask, because on a successful cancellation, the [AskOrder] is fully deleted from the contract.
 */
data class CancelAskResponse(
    val askId: String,
    val collateralReleased: Boolean,
    val cancelledAskOrder: AskOrder,
)
