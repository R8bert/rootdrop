<template>
  <div class="space-y-6">
    <!-- Header with Galaxy background -->
    <div class="relative overflow-hidden rounded-3xl p-8 text-center">
      <!-- Galaxy WebGL Background -->
      <div class="absolute inset-0 bg-black rounded-3xl">
        <Galaxy
          :focal="[0.5, 0.5]"
          :rotation="[1.0, 0.0]"
          :star-speed="0.2"
          :density="0.2"
          :hue-shift="10"
          :speed="0.2"
          :mouse-interaction="false"
          :glow-intensity="0.2"
          :saturation="0"
          :mouse-repulsion="true"
          :twinkle-intensity="4.5"
          :rotation-speed="0.1"
          :repulsion-strength="0"
          :transparent="true"
          class="w-full h-full"
        />
      </div>
      <div class="relative">
        <div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-black mb-4 shadow-lg">
          <IconSettings class="w-8 h-8 text-white" />
        </div>
        <h2 class="text-3xl font-bold mb-2 transition-colors duration-300 text-white">Admin Settings</h2>
        <p class="text-lg transition-colors duration-300 text-gray-300">Configure and customize your application</p>
      </div>
    </div>

    <form @submit.prevent="saveSettings" class="space-y-6">
      <!-- Maximum Validity Setting -->
      <div class="border rounded-2xl p-6 transition-all duration-300 hover:shadow-lg"
           :class="isDark ? 'bg-gray-800/50 border-gray-700/50 hover:border-blue-500/50' : 'bg-white border-gray-200 hover:border-blue-300'">
        <div class="flex items-center mb-6">
          <div class="p-3 rounded-xl mr-3"
               :style="{ backgroundColor: isDark ? '#1e3a8a' : '#dbeafe' }">
            <IconClock class="w-6 h-6"
                       :style="{ color: isDark ? '#60a5fa' : '#2563eb' }" />
          </div>
          <div>
            <h3 class="text-lg font-semibold"
                :class="isDark ? 'text-gray-100' : 'text-gray-900'">
              Maximum File Validity
            </h3>
            <p class="text-sm opacity-70"
               :class="isDark ? 'text-gray-400' : 'text-gray-600'">Set the maximum expiration period for uploads</p>
          </div>
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-5 gap-3">
          <label
            v-for="option in validityOptions"
            :key="option.value"
            :class="[
              'relative flex items-center justify-center px-4 py-3 rounded-lg border cursor-pointer transition-all duration-200',
              settings.maxValidity === option.value
                ? (isDark 
                    ? 'border-blue-500 bg-blue-600/20 text-blue-300' 
                    : 'border-blue-500 bg-blue-50 text-blue-700')
                : (isDark 
                    ? 'border-gray-600 bg-gray-700/30 text-gray-200 hover:border-gray-500' 
                    : 'border-gray-200 bg-gray-50 text-gray-700 hover:border-gray-300')
            ]"
          >
            <input
              type="radio"
              v-model="settings.maxValidity"
              :value="option.value"
              class="sr-only"
            />
            <div class="text-center">
              <div class="font-medium">{{ option.label }}</div>
              <div class="text-xs opacity-75">{{ option.description }}</div>
            </div>
          </label>
        </div>
        <p class="text-sm mt-3 opacity-70"
           :class="isDark ? 'text-gray-300' : 'text-gray-600'">
          Users will not be able to set file expiration longer than this maximum period.
        </p>
      </div>

      <!-- User Registration Setting -->
      <div class="border rounded-2xl p-6 transition-all duration-300 hover:shadow-lg"
           :class="isDark ? 'bg-gray-800/50 border-gray-700/50 hover:border-green-500/50' : 'bg-white border-gray-200 hover:border-green-300'">
        <div class="flex items-center mb-4">
          <div class="p-3 rounded-xl mr-3"
               :style="{ backgroundColor: isDark ? '#065f46' : '#d1fae5' }">
            <IconUserGroup class="w-6 h-6"
                           :style="{ color: isDark ? '#34d399' : '#059669' }" />
          </div>
          <h3 class="text-lg font-semibold"
              :class="isDark ? 'text-gray-100' : 'text-gray-900'">
            User Registration
          </h3>
        </div>
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium"
               :class="isDark ? 'text-gray-200' : 'text-gray-900'">Allow New User Registration</p>
            <p class="text-sm opacity-70"
               :class="isDark ? 'text-gray-400' : 'text-gray-600'">When disabled, only existing users and admins can log in</p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input
              type="checkbox"
              v-model="settings.allowRegistration"
              class="sr-only peer"
            />
            <div 
              :class="[
                'w-11 h-6 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300/50 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[\'\'] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600',
                isDark ? 'bg-gray-600 after:border-gray-300' : 'bg-gray-300 after:border-gray-200'
              ]"
            ></div>
          </label>
        </div>
      </div>

      <!-- File Expiration Behavior Setting -->
      <div class="border rounded-2xl p-6 transition-all duration-300 hover:shadow-lg"
           :class="isDark ? 'bg-gray-800/50 border-gray-700/50 hover:border-orange-500/50' : 'bg-white border-gray-200 hover:border-orange-300'">
        <div class="flex items-center mb-4">
          <div class="p-3 rounded-xl mr-3"
               :style="{ backgroundColor: isDark ? '#7c2d12' : '#fed7aa' }">
            <IconTrash class="w-6 h-6"
                       :style="{ color: isDark ? '#fb923c' : '#ea580c' }" />
          </div>
          <h3 class="text-lg font-semibold"
              :class="isDark ? 'text-gray-100' : 'text-gray-900'">
            File Expiration Behavior
          </h3>
        </div>
        <p class="text-sm mb-4 opacity-70"
           :class="isDark ? 'text-gray-400' : 'text-gray-600'">
          Choose what happens to files when they expire
        </p>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <label
            :class="[
              'relative flex flex-col p-4 rounded-lg border cursor-pointer transition-all duration-200',
              settings.expirationAction === 'unavailable'
                ? (isDark 
                    ? 'border-blue-500 bg-blue-600/20 text-blue-300'
                    : 'border-blue-500 bg-blue-50 text-blue-700')
                : (isDark 
                    ? 'border-gray-600 bg-gray-700/30 text-gray-200 hover:border-gray-500' 
                    : 'border-gray-200 bg-gray-50 text-gray-700 hover:border-gray-300')
            ]"
          >
            <input
              type="radio"
              v-model="settings.expirationAction"
              value="unavailable"
              class="sr-only"
            />
            <div class="flex items-center mb-2">
              <IconEyeOff class="w-5 h-5 mr-2" />
              <span class="font-medium">Make Unavailable</span>
            </div>
            <p class="text-sm opacity-70">Files become inaccessible but remain on the server</p>
          </label>
          
          <label
            :class="[
              'relative flex flex-col p-4 rounded-lg border cursor-pointer transition-all duration-200',
              settings.expirationAction === 'delete'
                ? (isDark 
                    ? 'border-blue-500 bg-blue-600/20 text-blue-300'
                    : 'border-blue-500 bg-blue-50 text-blue-700')
                : (isDark 
                    ? 'border-gray-600 bg-gray-700/30 text-gray-200 hover:border-gray-500' 
                    : 'border-gray-200 bg-gray-50 text-gray-700 hover:border-gray-300')
            ]"
          >
            <input
              type="radio"
              v-model="settings.expirationAction"
              value="delete"
              class="sr-only"
            />
            <div class="flex items-center mb-2">
              <IconDelete class="w-5 h-5 mr-2" />
              <span class="font-medium">Delete Files</span>
            </div>
            <p class="text-sm opacity-70">Files are permanently removed from the server</p>
          </label>
        </div>
        <div class="mt-4 p-3 rounded-lg border"
             :class="isDark ? 'bg-yellow-900/20 border-yellow-700/30 text-yellow-200' : 'bg-yellow-50 border-yellow-200 text-yellow-800'">
          <p class="text-sm">
            <strong>Note:</strong> This setting affects how the system handles files after they expire.
            The cleanup process runs every hour.
          </p>
        </div>
      </div>

      <!-- Upload Settings -->
      <div class="relative group">
        <div class="absolute inset-0 bg-gradient-to-r from-green-600/20 to-emerald-600/20 rounded-2xl blur-xl group-hover:blur-2xl transition-all duration-300"></div>
        <div class="relative backdrop-blur-xl border rounded-2xl p-6 transition-all duration-300"
             :style="{
               backgroundColor: isDark ? 'rgba(255,255,255,0.05)' : 'rgba(255,255,255,0.8)',
               borderColor: isDark ? 'rgba(16, 185, 129, 0.3)' : 'rgba(16, 185, 129, 0.2)'
             }">
          <h3 class="text-lg font-semibold mb-4 flex items-center transition-colors duration-300"
              :style="{ color: isDark ? '#d1fae5' : '#111827' }">
            <IconCloudArrowUp class="w-5 h-5 mr-2"
                              :style="{ color: isDark ? '#34d399' : '#059669' }" />
            Upload Settings
          </h3>
          <div class="max-w-md">
            <div>
              <label class="block text-sm font-medium mb-2 transition-colors duration-300"
                     :style="{ color: isDark ? '#d1fae5' : '#374151' }">
                Maximum Upload Size
              </label>
              <select
                v-model="settings.maxUploadSize"
                class="w-full px-3 py-2 border rounded-xl focus:ring-2 focus:ring-green-500 focus:border-green-500 transition-all duration-300 backdrop-blur-sm"
                :style="{
                  backgroundColor: isDark ? 'rgba(255,255,255,0.1)' : 'rgba(255,255,255,0.8)',
                  borderColor: isDark ? 'rgba(16, 185, 129, 0.3)' : 'rgba(16, 185, 129, 0.2)',
                  color: isDark ? '#f9fafb' : '#111827'
                }"
              >
                <option value="1048576">1 MB</option>
                <option value="5242880">5 MB</option>
                <option value="10485760">10 MB</option>
                <option value="26214400">25 MB</option>
                <option value="52428800">50 MB</option>
                <option value="104857600">100 MB</option>
                <option value="262144000">250 MB</option>
                <option value="524288000">500 MB</option>
                <option value="1073741824">1 GB</option>
                <option value="2147483648">2 GB</option>
                <option value="5368709120">5 GB</option>
                <option value="10737418240">10 GB</option>
                <option value="-1">Unlimited</option>
              </select>
              <p class="text-sm mt-1 transition-colors duration-300 opacity-80"
                 :style="{ color: isDark ? '#d1d5db' : '#4b5563' }">
                Set the maximum file size users can upload. Choose "Unlimited" to allow any size.
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Branding Settings -->
      <div class="border rounded-2xl p-6 transition-all duration-300 hover:shadow-lg"
           :class="isDark ? 'bg-gray-800/50 border-gray-700/50 hover:border-purple-500/50' : 'bg-white border-gray-200 hover:border-purple-300'">
        <div class="flex items-center mb-6">
          <div class="p-3 rounded-xl mr-3"
               :style="{ backgroundColor: isDark ? '#581c87' : '#f3e8ff' }">
            <IconPhoto class="w-6 h-6"
                       :style="{ color: isDark ? '#c084fc' : '#9333ea' }" />
          </div>
          <div>
            <h3 class="text-lg font-semibold"
                :class="isDark ? 'text-gray-100' : 'text-gray-900'">
              Branding & Identity
            </h3>
            <p class="text-sm opacity-70"
               :class="isDark ? 'text-gray-400' : 'text-gray-600'">Customize your application's look and feel</p>
          </div>
        </div>
        <div class="space-y-6">
          <!-- Application Title -->
          <div>
            <label class="block text-sm font-medium mb-2"
                   :class="isDark ? 'text-gray-200' : 'text-gray-700'">
              Application Title
            </label>
            <input
              type="text"
              v-model="settings.navbarTitle"
              placeholder="PinGO"
              class="w-full px-4 py-2 border rounded-lg transition-all duration-200 focus:ring-2 focus:ring-blue-500/50"
              :class="isDark 
                ? 'bg-gray-700/50 border-gray-600 text-gray-100 focus:border-blue-500' 
                : 'bg-white border-gray-300 text-gray-900 focus:border-blue-500'"
            />
            <p class="text-sm mt-1 opacity-70"
               :class="isDark ? 'text-gray-400' : 'text-gray-500'">
              This title will appear next to the logo in the navigation bar.
            </p>
          </div>
          
          <!-- Logo and Background Images -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label class="block text-sm font-medium mb-2"
                     :class="isDark ? 'text-gray-200' : 'text-gray-700'">
                Logo
              </label>
              <div class="flex items-center space-x-4">
                <img
                  v-if="settings.logo"
                  :src="getAssetUrl(settings.logo)"
                  alt="Current logo"
                  class="w-12 h-12 object-contain rounded-lg border"
                  :class="isDark ? 'border-gray-600' : 'border-gray-300'"
                />
                <input
                  type="file"
                  ref="logoInput"
                  @change="handleLogoUpload"
                  accept="image/*"
                  class="block w-full text-sm file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:bg-blue-600 file:text-white hover:file:bg-blue-700 transition-all duration-200"
                  :class="isDark ? 'text-gray-300' : 'text-gray-600'"
                />
              </div>
            </div>
            <div>
              <label class="block text-sm font-medium mb-2"
                     :class="isDark ? 'text-gray-200' : 'text-gray-700'">
                Background Image
              </label>
              <div class="flex items-center space-x-4">
                <img
                  v-if="settings.backgroundImage"
                  :src="getAssetUrl(settings.backgroundImage)"
                  alt="Current background"
                  class="w-12 h-12 object-cover rounded-lg border"
                  :class="isDark ? 'border-gray-600' : 'border-gray-300'"
                />
                <input
                  type="file"
                  ref="backgroundInput"
                  @change="handleBackgroundUpload"
                  accept="image/*"
                  class="block w-full text-sm file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:bg-blue-600 file:text-white hover:file:bg-blue-700 transition-all duration-200"
                  :class="isDark ? 'text-gray-300' : 'text-gray-600'"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Color Customization Settings -->
  

      <!-- Save Button -->
      <div class="flex justify-end pt-4">
        <button
          type="submit"
          :disabled="isLoading"
          class="group relative px-10 py-4 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-bold rounded-2xl transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg hover:shadow-xl hover:scale-105 overflow-hidden"
        >
          <div class="absolute inset-0 bg-white/20 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
          <div class="relative flex items-center">
            <IconSave v-if="!isLoading" class="w-6 h-6 mr-2" />
            <div v-else class="w-6 h-6 mr-2 animate-spin rounded-full border-3 border-white border-t-transparent"></div>
            {{ isLoading ? 'Saving Changes...' : 'Save All Settings' }}
          </div>
        </button>
      </div>
    </form>

    <!-- Success Message -->
    <div
      v-if="showSuccess"
      class="fixed bottom-4 right-4 z-50 transition-all duration-300"
    >
      <div class="px-6 py-3 rounded-xl border shadow-lg"
           :class="isDark 
             ? 'bg-green-900/20 border-green-500/30 text-green-300' 
             : 'bg-green-50 border-green-200 text-green-800'">
        <div class="flex items-center">
          <IconCheckCircle class="w-5 h-5 mr-2" />
          <span class="font-medium">Settings saved successfully!</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuth } from '../../../composables/useAuth'
import { useTheme } from '../../../composables/useTheme'
import { getAssetUrl } from '../../../utils/apiUtils'
import Galaxy from '../../../blocks/Backgrounds/Galaxy/Galaxy.vue'
import IconSettings from '~icons/heroicons/cog-8-tooth'
import IconClock from '~icons/heroicons/clock'
import IconUserGroup from '~icons/heroicons/user-group'
import IconCloudArrowUp from '~icons/heroicons/cloud-arrow-up'
import IconPhoto from '~icons/heroicons/photo'
import IconSave from '~icons/heroicons/arrow-down-tray'
import IconCheckCircle from '~icons/heroicons/check-circle'
import IconTrash from '~icons/heroicons/trash'
import IconEyeOff from '~icons/heroicons/eye-slash'
import IconDelete from '~icons/heroicons/x-circle'

const { isDark } = useTheme()

const { saveAdminSettings, getSettings, isLoading } = useAuth()

const settings = ref({
  logo: '',
  backgroundImage: '',
  navbarTitle: 'PinGO',
  maxUploadSize: 104857600,
  blurIntensity: 0,
  maxValidity: '7days',
  allowRegistration: true,
  expirationAction: 'unavailable'
})

const logoFile = ref<File | null>(null)
const backgroundFile = ref<File | null>(null)
const showSuccess = ref(false)

const validityOptions = [
  { value: '7days', label: '7 Days', description: 'One week' },
  { value: '1month', label: '1 Month', description: '30 days' },
  { value: '6months', label: '6 Months', description: 'Half year' },
  { value: '1year', label: '1 Year', description: '12 months' },
  { value: 'never', label: 'Never', description: 'Permanent' }
]

const handleLogoUpload = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    logoFile.value = target.files[0]
  }
}

const handleBackgroundUpload = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    backgroundFile.value = target.files[0]
  }
}

const saveSettings = async () => {
  try {
    console.log('💾 Saving settings:', settings.value)
    
    const formData = new FormData()
    
    formData.append('navbarTitle', settings.value.navbarTitle)
    formData.append('maxUploadSize', settings.value.maxUploadSize.toString())
    formData.append('blurIntensity', settings.value.blurIntensity.toString())
    formData.append('maxValidity', settings.value.maxValidity)
    formData.append('allowRegistration', settings.value.allowRegistration.toString())
    formData.append('expirationAction', settings.value.expirationAction)
    
    if (logoFile.value) {
      formData.append('logo', logoFile.value)
    }
    
    if (backgroundFile.value) {
      formData.append('backgroundImage', backgroundFile.value)
    }
    
    const result = await saveAdminSettings(formData)
    console.log('✅ Settings saved successfully. Response:', result)
    
    // Update local settings with response
    settings.value = { ...settings.value, ...result }
    
    // Show success message
    showSuccess.value = true
    setTimeout(() => {
      showSuccess.value = false
    }, 3000)
    
    // Dispatch event to notify navbar of settings update
    window.dispatchEvent(new CustomEvent('settings-updated'))
    
    // Reset file inputs
    logoFile.value = null
    backgroundFile.value = null
    
  } catch (error: any) {
    console.error('❌ Failed to save settings:', error)
    alert('Failed to save settings: ' + error.message)
  }
}

const loadSettings = async () => {
  try {
    const currentSettings = await getSettings()
    console.log('📥 RAW settings from API:', JSON.stringify(currentSettings, null, 2))
    console.log('📊 Field by field:')
    console.log('  - logo:', currentSettings.logo, '(type:', typeof currentSettings.logo, ')')
    console.log('  - backgroundImage:', currentSettings.backgroundImage, '(type:', typeof currentSettings.backgroundImage, ')')
    console.log('  - navbarTitle:', currentSettings.navbarTitle, '(type:', typeof currentSettings.navbarTitle, ')')
    console.log('  - maxUploadSize:', currentSettings.maxUploadSize, '(type:', typeof currentSettings.maxUploadSize, ')')
    console.log('  - blurIntensity:', currentSettings.blurIntensity, '(type:', typeof currentSettings.blurIntensity, ')')
    console.log('  - maxValidity:', currentSettings.maxValidity, '(type:', typeof currentSettings.maxValidity, ')')
    console.log('  - allowRegistration:', currentSettings.allowRegistration, '(type:', typeof currentSettings.allowRegistration, ')')
    console.log('  - expirationAction:', currentSettings.expirationAction, '(type:', typeof currentSettings.expirationAction, ')')
    
    // Backend returns camelCase due to serde rename
    settings.value = {
      logo: currentSettings.logo ?? '',
      backgroundImage: currentSettings.backgroundImage ?? '',
      navbarTitle: currentSettings.navbarTitle ?? 'PinGO',
      maxUploadSize: currentSettings.maxUploadSize ?? 104857600,
      blurIntensity: currentSettings.blurIntensity ?? 0,
      maxValidity: currentSettings.maxValidity ?? '7days',
      allowRegistration: currentSettings.allowRegistration ?? true,
      expirationAction: currentSettings.expirationAction ?? 'unavailable'
    }
    
    console.log('✅ Final settings object:', JSON.stringify(settings.value, null, 2))
  } catch (error) {
    console.error('❌ Failed to load settings:', error)
  }
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.slider::-webkit-slider-thumb {
  appearance: none;
  height: 20px;
  width: 20px;
  border-radius: 50%;
  background: #3b82f6;
  cursor: pointer;
  border: 2px solid #ffffff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.slider::-moz-range-thumb {
  height: 20px;
  width: 20px;
  border-radius: 50%;
  background: #3b82f6;
  cursor: pointer;
  border: 2px solid #ffffff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

/* Dark mode slider track */
[data-theme="dark"] .slider::-webkit-slider-track {
  background: #4b5563;
}

[data-theme="dark"] .slider::-moz-range-track {
  background: #4b5563;
}

/* Dropdown/select styling for dark mode compatibility */
select option {
  background-color: inherit;
  color: inherit;
}
</style>
