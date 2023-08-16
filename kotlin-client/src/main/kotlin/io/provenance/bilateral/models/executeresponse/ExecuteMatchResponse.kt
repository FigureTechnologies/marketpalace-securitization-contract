package io.provenance.bilateral.models.executeresponse

/**
 * The data response returned after a match between an ask and bid is successfully made.
 *
 * @param askId The unique identifier of the matched ask.
 * @param bidId The unique identifier of the matched bid.
 * @param askDeleted Whether or not the ask was deleted after the match was completed.  In most cases, the ask is
 * deleted when a match is made, because no more information about the ask is needed within the contract.
 * @param bidDeleted Whether or not the bid was deleted after the match was completed.  A bid may remain if a marker share
 * sale ask matches with a bid that requests more shares than the ask is configured to provide.
 * @param collateralReleased Whether or not the collateral held on behalf of the asker was released to the target party.
 * In the case of a non-marker share sale, the collateral should always be released upon the completion of a match.  In
 * a marker share sale, however, the collateral is only released when the ask has been deleted and there are no other
 * outstanding asks for the same marker.  This value being false does not mean that the trade did not complete successfully:
 * even if this value is false, the proper amount of coin can be expected to have been sent to the correct parties. This
 * flag only indicates that a held marker or scope was transferred out of the contract's ownership to the appropriate
 * party.
 */
data class ExecuteMatchResponse(
    val askId: String,
    val bidId: String,
    val askDeleted: Boolean,
    val bidDeleted: Boolean,
    val collateralReleased: Boolean,
)
