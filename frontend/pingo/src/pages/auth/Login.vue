<template>
  <div class="min-h-screen flex relative overflow-hidden">
    <!-- Interactive Galaxy Background -->
    <div class="fixed inset-0">
      <Galaxy 
        v-if="galaxyLoaded"
        :mouse-repulsion="false"
        :mouse-interaction="false"
        :density="1.5"
        :glow-intensity="0.25"
        :saturation="0.1"
        :hue-shift="120"
      />
    </div>

    <!-- Main Container -->
    <div class="w-full flex items-center justify-center p-4 lg:p-8 relative z-10">
      <div class="w-full max-w-md p-8 lg:p-10 rounded-3xl shadow-2xl border relative overflow-hidden backdrop-blur-xl"
        :class="isDark ? 'bg-neutral-900/90 border-white/10 shadow-blue-500/10' : 'bg-white/90 border-neutral-200/50 shadow-neutral-500/10'">
        
        <!-- Gradient accent -->
        <div class="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-neutral-800 via-neutral-600 to-neutral-800 animate-gradient"></div>
        
        <!-- Floating orbs -->
        <div class="absolute -top-20 -right-20 w-40 h-40 bg-neutral-500/10 rounded-full blur-3xl animate-float"></div>
        <div class="absolute -bottom-20 -left-20 w-40 h-40 bg-neutral-600/10 rounded-full blur-3xl animate-float" style="animation-delay: 2s;"></div>
        
        <div class="relative z-10">
          <!-- Logo/Brand -->
          <div class="mb-10 text-center animate-fade-in">
            <div class="inline-flex items-center justify-center w-16 h-16 mb-4 rounded-2xl bg-gradient-to-br from-neutral-800 to-neutral-900 shadow-lg shadow-neutral-900/30">
              <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
              </svg>
            </div>
            <h2 class="text-3xl font-bold tracking-tight transition-colors duration-300"
              :class="isDark ? 'text-white' : 'text-neutral-900'">
              Welcome Back
            </h2>
            <p class="text-sm mt-2 transition-colors duration-300" 
              :class="isDark ? 'text-neutral-400' : 'text-neutral-600'">
              Sign in to continue to RootDrop
            </p>
          </div>
          <!-- Login Form -->
          <form @submit.prevent="handleLogin" class="space-y-6 animate-fade-in" style="animation-delay: 0.1s;">
            <!-- Username or Email Field -->
            <div class="space-y-3">
              <label class="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider mb-2"
                :class="isDark ? 'text-neutral-300' : 'text-neutral-700'">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                </svg>
                Username or Email
              </label>
              <input
                v-model="usernameOrEmail"
                type="text"
                required
                :class="[
                  'w-full px-4 py-3 border rounded-xl focus:outline-none focus:ring-2 transition-all duration-300',
                  isDark
                    ? 'bg-neutral-800/50 border-neutral-700 text-white focus:ring-neutral-500/50 focus:border-neutral-500 placeholder-neutral-500'
                    : 'bg-white border-neutral-300 text-neutral-900 focus:ring-neutral-400/50 focus:border-neutral-500 placeholder-neutral-400'
                ]"
                placeholder="Enter your username or email"
              />
            </div>

            <!-- Password Field -->
            <div class="space-y-3">
              <label class="flex items-center gap-2 text-xs font-semibold uppercase tracking-wider mb-2"
                :class="isDark ? 'text-neutral-300' : 'text-neutral-700'">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"></path>
                </svg>
                Password
              </label>
              <div class="relative">
                <input
                  v-model="password"
                  :type="showPassword ? 'text' : 'password'"
                  required
                  :class="[
                    'w-full px-4 py-3 pr-12 border rounded-xl focus:outline-none focus:ring-2 transition-all duration-300',
                    isDark
                      ? 'bg-neutral-800/50 border-neutral-700 text-white focus:ring-neutral-500/50 focus:border-neutral-500 placeholder-neutral-500'
                      : 'bg-white border-neutral-300 text-neutral-900 focus:ring-neutral-400/50 focus:border-neutral-500 placeholder-neutral-400'
                  ]"
                  placeholder="••••••••"
                />
                <button
                  @click="showPassword = !showPassword"
                  type="button"
                  class="absolute right-3 top-1/2 -translate-y-1/2 p-1.5 rounded-lg transition-colors hover:bg-neutral-200 dark:hover:bg-neutral-700"
                  :class="isDark ? 'text-neutral-400' : 'text-neutral-500'"
                >
                  <svg v-if="showPassword" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
                  </svg>
                  <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"></path>
                  </svg>
                </button>
              </div>
            </div>

            <!-- Error Message -->
            <div v-if="errorMessage" 
              class="p-4 border rounded-xl animate-fade-in"
              :class="isDark ? 'bg-red-900/50 border-red-700/50 text-red-200' : 'bg-red-50 border-red-200 text-red-600'">
              <p class="text-sm">{{ errorMessage }}</p>
            </div>

            <!-- Submit Button -->
            <button
              type="submit"
              :disabled="isLoading"
              class="group relative w-full overflow-hidden bg-gradient-to-r from-neutral-800 via-neutral-700 to-neutral-800 bg-[length:200%_100%] hover:bg-right-bottom disabled:from-neutral-500 disabled:to-neutral-600 disabled:bg-[length:100%_100%] text-white font-semibold py-3.5 px-6 rounded-xl transition-all duration-500 transform hover:scale-[1.02] hover:shadow-xl hover:shadow-neutral-900/40 active:scale-[0.98] disabled:cursor-not-allowed disabled:transform-none disabled:opacity-50 mt-6"
            >
              <span class="relative z-10 flex items-center justify-center gap-2">
                <svg v-if="isLoading" class="w-5 h-5 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                </svg>
                <svg v-else class="w-5 h-5 transition-transform group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path>
                </svg>
                {{ isLoading ? 'Signing In...' : 'Sign In' }}
              </span>
              <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent translate-x-[-200%] group-hover:translate-x-[200%] transition-transform duration-1000"></div>
            </button>
          </form>

          <!-- Register Link -->
          <div class="text-center pt-6 animate-fade-in" style="animation-delay: 0.2s;">
            <p class="text-sm" :class="isDark ? 'text-neutral-400' : 'text-neutral-600'">
              Don't have an account?
              <button
                @click="switchToRegister"
                class="font-semibold ml-1 transition-all duration-200 hover:underline"
                :class="isDark ? 'text-neutral-300 hover:text-white' : 'text-neutral-700 hover:text-neutral-900'"
              >
                Sign up
              </button>
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template><script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuth } from '../../composables/useAuth'
import { useTheme } from '../../composables/useTheme'
import { useRouter } from 'vue-router'
import Galaxy from '../../blocks/Backgrounds/Galaxy/Galaxy.vue'

const emit = defineEmits<{
  switchToRegister: []
}>()

const { login, isLoading } = useAuth()
const { isDark } = useTheme()
const router = useRouter()

const usernameOrEmail = ref('')
const password = ref('')
const errorMessage = ref('')
const showPassword = ref(false)
const galaxyLoaded = ref(false)

onMounted(() => {
  galaxyLoaded.value = true
})

const handleLogin = async () => {
  errorMessage.value = ''

  if (!usernameOrEmail.value || !password.value) {
    errorMessage.value = 'Please fill in all fields'
    return
  }

  const result = await login(usernameOrEmail.value, password.value)

  if (result.success) {
    router.push('/')
  } else {
    errorMessage.value = result.message
  }
}

const switchToRegister = () => {
  emit('switchToRegister')
}
</script>

<style scoped>
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px) rotate(0deg);
  }
  50% {
    transform: translateY(-20px) rotate(5deg);
  }
}

@keyframes gradient {
  0%, 100% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
}

.animate-fade-in {
  animation: fade-in 0.6s ease-out forwards;
  opacity: 0;
}

.animate-float {
  animation: float 6s ease-in-out infinite;
}

.animate-gradient {
  background-size: 200% 200%;
  animation: gradient 3s ease infinite;
}
</style>
