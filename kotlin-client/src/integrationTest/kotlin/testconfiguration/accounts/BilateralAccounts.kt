package testconfiguration.accounts

import org.bouncycastle.jce.provider.BouncyCastleProvider
import java.security.Security

object BilateralAccounts {
    init {
        // Enable bouncycastle provider to prevent hdwallet utils from falling over
        Security.addProvider(BouncyCastleProvider())
    }

    val askerAccount: MnemonicSigner by lazy {
        MnemonicSigner.new("figure razor approve praise hope school project question neglect other sail spray sense shine page test oil purchase fox neither orphan birth easy mobile")
    }

    val bidderAccount: MnemonicSigner by lazy {
        MnemonicSigner.new("letter arch fragile sport tell ill hunt celery bamboo affair click dinner submit merge lottery either ribbon sadness sand zoo flush grab symbol cat")
    }

    val adminAccount: MnemonicSigner by lazy {
        MnemonicSigner.new("idle call jealous poet correct book trust gold fringe retire hard fall champion shiver super ginger night double crawl topic impose globe antique student")
    }

    val fundingAccount: MnemonicSigner by lazy {
        MnemonicSigner.new(
            mnemonic = "stable payment cliff fault abuse clinic bus belt film then forward world goose bring picnic rich special brush basic lamp window coral worry change",
            hdPath = "m/44'/1'/0'/0/0",
        )
    }
}
