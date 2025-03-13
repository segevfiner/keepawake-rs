package io.github.segevfiner.keepawake

class NativeLib {
    companion object {
        // Used to load the 'keepawake' library on application startup.
        init {
            System.loadLibrary("keepawake_android")
        }

        /**
         * A native method that is implemented by the 'keepawake' native library,
         * which is packaged with this application.
         */
        @JvmStatic
        external fun stringFromJNI(): String
    }
}