package com.rustlibs.authentication

import android.util.Log
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity

class Authentication (private  val activity: FragmentActivity) {

    fun authenticate() {
        val executor = ContextCompat.getMainExecutor(activity)
        val prompt = BiometricPrompt(
            activity, executor,
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    Log.d("AuthHelper", "Success")
                    // TODO: JNI経由でRustに結果を通知
                }

                override fun onAuthenticationFailed() {
                    Log.d("AuthHelper", "Failed")
                }
                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    Log.d("AuthHelper", "Error: $errString")
                }
            }
        )
        val info = BiometricPrompt.PromptInfo.Builder()
            .setTitle("認証")
            .setSubtitle("本人確認をしてください")
            .setNegativeButtonText("キャンセル")
            .build()

        prompt.authenticate(info)
    }
}