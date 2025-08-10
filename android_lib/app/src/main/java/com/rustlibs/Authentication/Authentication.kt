package com.rustlibs.authentication

import android.app.Activity
import android.util.Log
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat

object Authentication {

    @JvmStatic
    fun authenticate(activity: Activity) {
        val executor = ContextCompat.getMainExecutor(activity)
        val prompt = BiometricPrompt(
            activity as androidx.fragment.app.FragmentActivity,
            executor,
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    Log.d("AuthHelper", "Success")
                    nativeOnAuthResult(true)
                }
                override fun onAuthenticationFailed() {
                    Log.d("AuthHelper", "Failed")
                    nativeOnAuthResult(false)
                }
                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    Log.d("AuthHelper", "Error: $errString")
                    nativeOnAuthResult(false)
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

    // JNIでRustに結果を返す
    external fun nativeOnAuthResult(success: Boolean)
}