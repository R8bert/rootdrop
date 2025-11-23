<template>
  <div class="min-h-screen pt-20 transition-colors duration-300"
       :style="{ backgroundColor: isDark ? '#0a0a0a' : '#f8fafc' }">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <!-- Header Section -->
      <div class="text-center mb-12">
        <div class="relative inline-block mb-6">
          <div class="w-24 h-24 rounded-2xl overflow-hidden shadow-lg from-blue-500 to-indigo-600 flex items-center justify-center">
            <img 
              v-if="user?.avatar" 
              :src="getAssetUrl(user.avatar)" 
              :alt="user.username"
              class="w-full h-full object-cover"
              @error="handleAvatarError"
            />
            <IconUser v-else class="w-12 h-12 text-white" />
          </div>
          <button 
            @click="openAvatarUpload"
            class="absolute -bottom-2 -right-2 w-8 h-8 bg-blue-600 hover:bg-blue-700 text-white rounded-full flex items-center justify-center shadow-lg transition-colors"
            title="Change avatar"
          >
            <IconCamera class="w-4 h-4" />
          </button>
          <input 
            ref="avatarInput" 
            type="file" 
            accept=".png,.jpg,.jpeg,.gif,image/png,image/jpeg,image/gif" 
            @change="handleAvatarUpload" 
            class="hidden"
          />
        </div>
        <p class="text-xl mb-4 transition-colors duration-300"
           :style="{ color: isDark ? '#e5e7eb' : '#4b5563' }">{{ user?.username }}</p>
        <div class="inline-flex items-center space-x-4 text-sm transition-colors duration-300"
             :style="{ color: isDark ? '#9ca3af' : '#6b7280' }">
          <span class="flex items-center">
            <IconCalendar class="w-4 h-4 mr-1" />
            Member since {{ formatDate(user?.created_at) }}
          </span>
          <span v-if="isAdmin" class="flex items-center text-blue-600">
            <IconShieldCheck class="w-4 h-4 mr-1" />
            Administrator
          </span>
        </div>
      </div>

      <!-- Stats Cards -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div class="rounded-2xl p-6 border shadow-sm hover:shadow-md transition-all duration-300"
             :style="{ 
               backgroundColor: isDark ? '#1a1a1a' : '#ffffff',
               borderColor: isDark ? '#374151' : '#e5e7eb'
             }">
          <div class="flex items-center">
            <div class="p-3 rounded-xl transition-colors duration-300"
                 :style="{ backgroundColor: isDark ? '#1e3a8a' : '#dbeafe' }">
              <IconCloudArrowUp class="w-6 h-6 transition-colors duration-300"
                                :style="{ color: isDark ? '#60a5fa' : '#2563eb' }" />
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium transition-colors duration-300"
                 :style="{ color: isDark ? '#9ca3af' : '#4b5563' }">Total Uploads</p>
              <p class="text-2xl font-bold transition-colors duration-300"
                 :style="{ color: isDark ? '#f9fafb' : '#111827' }">{{ uploads.length }}</p>
            </div>
          </div>
        </div>

        <div class="rounded-2xl p-6 border shadow-sm hover:shadow-md transition-all duration-300"
             :style="{ 
               backgroundColor: isDark ? '#1a1a1a' : '#ffffff',
               borderColor: isDark ? '#374151' : '#e5e7eb'
             }">
          <div class="flex items-center">
            <div class="p-3 rounded-xl transition-colors duration-300"
                 :style="{ backgroundColor: isDark ? '#059669' : '#dcfce7' }">
              <IconArchiveBox class="w-6 h-6 transition-colors duration-300"
                              :style="{ color: isDark ? '#34d399' : '#16a34a' }" />
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium transition-colors duration-300"
                 :style="{ color: isDark ? '#9ca3af' : '#4b5563' }">Total Size</p>
              <p class="text-2xl font-bold transition-colors duration-300"
                 :style="{ color: isDark ? '#f9fafb' : '#111827' }">{{ formatTotalSize() }}</p>
            </div>
          </div>
        </div>

        <div class="rounded-2xl p-6 border shadow-sm hover:shadow-md transition-all duration-300"
             :style="{ 
               backgroundColor: isDark ? '#1a1a1a' : '#ffffff',
               borderColor: isDark ? '#374151' : '#e5e7eb'
             }">
          <div class="flex items-center">
            <div class="p-3 rounded-xl transition-colors duration-300"
                 :style="{ backgroundColor: isDark ? '#7c3aed' : '#f3e8ff' }">
              <IconClock class="w-6 h-6 transition-colors duration-300"
                         :style="{ color: isDark ? '#a78bfa' : '#7c3aed' }" />
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium transition-colors duration-300"
                 :style="{ color: isDark ? '#9ca3af' : '#4b5563' }">Active Files</p>
              <p class="text-2xl font-bold transition-colors duration-300"
                 :style="{ color: isDark ? '#f9fafb' : '#111827' }">{{ activeUploads.length }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Main Content Tabs -->
      <div class="rounded-2xl border shadow-sm transition-all duration-300"
           :style="{ 
             backgroundColor: isDark ? '#1a1a1a' : '#ffffff',
             borderColor: isDark ? '#374151' : '#e5e7eb'
           }">
        <!-- Tab Navigation -->
        <div class="border-b transition-colors duration-300"
             :style="{ borderColor: isDark ? '#374151' : '#e5e7eb' }">
          <nav class="flex space-x-4 sm:space-x-8 px-4 sm:px-6 overflow-x-auto scrollbar-hide">
            <button
              @click="activeTab = 'uploads'"
              class="py-4 px-1 border-b-2 font-medium text-xs sm:text-sm transition-colors duration-300 whitespace-nowrap flex-shrink-0"
              :style="{
                borderBottomColor: activeTab === 'uploads' ? '#3b82f6' : 'transparent',
                color: activeTab === 'uploads' 
                  ? '#3b82f6' 
                  : (isDark ? '#9ca3af' : '#6b7280')
              }"
            >
              <div class="flex items-center space-x-1 sm:space-x-2">
                <IconFolder class="w-3 h-3 sm:w-4 sm:h-4" />
                <span>My Uploads</span>
              </div>
            </button>
            <button
              @click="activeTab = 'reverse'"
              class="py-4 px-1 border-b-2 font-medium text-xs sm:text-sm transition-colors duration-300 whitespace-nowrap flex-shrink-0"
              :style="{
                borderBottomColor: activeTab === 'reverse' ? '#10b981' : 'transparent',
                color: activeTab === 'reverse' 
                  ? '#10b981' 
                  : (isDark ? '#9ca3af' : '#6b7280')
              }"
            >
              <div class="flex items-center space-x-1 sm:space-x-2">
                <IconShare class="w-3 h-3 sm:w-4 sm:h-4" />
                <span>Reverse Share</span>
              </div>
            </button>
            <button
              v-if="isAdmin"
              @click="activeTab = 'statistics'"
              class="py-4 px-1 border-b-2 font-medium text-xs sm:text-sm transition-colors duration-300 whitespace-nowrap flex-shrink-0"
              :style="{
                borderBottomColor: activeTab === 'statistics' ? '#8b5cf6' : 'transparent',
                color: activeTab === 'statistics' 
                  ? '#8b5cf6' 
                  : (isDark ? '#9ca3af' : '#6b7280')
              }"
            >
              <div class="flex items-center space-x-1 sm:space-x-2">
                <IconChartBar class="w-3 h-3 sm:w-4 sm:h-4" />
                <span>Statistics</span>
              </div>
            </button>
            <button
              v-if="isAdmin"
              @click="activeTab = 'users'"
              class="py-4 px-1 border-b-2 font-medium text-xs sm:text-sm transition-colors duration-300 whitespace-nowrap flex-shrink-0"
              :style="{
                borderBottomColor: activeTab === 'users' ? '#f59e0b' : 'transparent',
                color: activeTab === 'users' 
                  ? '#f59e0b' 
                  : (isDark ? '#9ca3af' : '#6b7280')
              }"
            >
              <div class="flex items-center space-x-2">
                <IconUsers class="w-4 h-4" />
                <span>Users</span>
              </div>
            </button>
            <button
              v-if="isAdmin"
              @click="activeTab = 'settings'"
              class="py-4 px-1 border-b-2 font-medium text-sm transition-colors duration-300"
              :style="{
                borderBottomColor: activeTab === 'settings' ? '#3b82f6' : 'transparent',
                color: activeTab === 'settings' 
                  ? '#3b82f6' 
                  : (isDark ? '#9ca3af' : '#6b7280')
              }"
            >
              <div class="flex items-center space-x-2">
                <IconCog class="w-4 h-4" />
                <span>Settings</span>
              </div>
            </button>
          </nav>
        </div>

        <!-- Tab Content -->
        <div class="p-6">
          <!-- Uploads Tab -->
          <div v-if="activeTab === 'uploads'">
            <UploadsTab
              :uploads="uploads"
              :is-loading="isLoading"
              :show-expiration-dropdown="showExpirationDropdown"
              @copy-to-clipboard="copyToClipboard"
              @toggle-availability="toggleAvailability"
              @change-expiration="changeExpiration"
              @toggle-expiration-dropdown="toggleExpirationDropdown"
              @delete-upload="handleDeleteUpload"
            />
          </div>

          <!-- Reverse Share Tab -->
          <div v-if="activeTab === 'reverse'">
            <ReverseShareTab
              :reverse-tokens="reverseTokens"
              :is-loading="isLoading"
              @create-reverse-token="createReverseToken"
              @delete-reverse-token="deleteReverseToken"
              @copy-to-clipboard="copyToClipboard"
            />
          </div>

          <!-- Statistics Tab -->
          <div v-if="activeTab === 'statistics' && isAdmin">
            <StatisticsTab
              :admin-stats="adminStats"
              :is-loading="isLoadingAdminSettings"
            />
          </div>

          <!-- Users Tab -->
          <div v-if="activeTab === 'users' && isAdmin">
            <UsersTab
              :admin-users="adminUsers"
              :is-loading="isLoadingAdminSettings"
              @toggle-user-block="toggleUserBlock"
            />
          </div>

          <!-- Settings Tab -->
          <div v-if="activeTab === 'settings' && isAdmin">
            <SettingsTab
              :is-admin="isAdmin"
              @settings-updated="fetchAdminData"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Success Message for Copy -->
    <div
      v-if="showCopySuccess"
      class="fixed bottom-4 right-4 px-6 py-3 rounded-lg shadow-lg z-50 transition-all duration-300"
      :style="{ 
        backgroundColor: isDark ? '#059669' : '#10b981',
        color: '#ffffff'
      }"
    >
      <div class="flex items-center">
        <IconClipboardDocument class="w-5 h-5 mr-2" />
        {{ successMessage }}
      </div>
    </div>
  </div>

  <!-- Avatar Upload Dialog -->
  <div v-if="showAvatarDialog" 
       class="fixed inset-0 z-50 flex items-center justify-center p-4 animate-dialog-backdrop"
       @click="closeAvatarDialog">
    <div class="fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm transition-opacity duration-300"></div>
    
    <div class="relative bg-white dark:bg-gray-800 rounded-3xl p-8 max-w-md w-full mx-4 shadow-2xl animate-dialog-content"
         :style="{ backgroundColor: isDark ? '#1a1a1a' : '#ffffff' }"
         @click.stop>
      
      <!-- Close button -->
      <button @click="closeAvatarDialog" 
              class="absolute top-4 right-4 w-8 h-8 rounded-full flex items-center justify-center transition-colors duration-200"
              :style="{ 
                backgroundColor: isDark ? '#374151' : '#f3f4f6',
                color: isDark ? '#9ca3af' : '#6b7280'
              }">
        <IconClose class="w-5 h-5" />
      </button>

      <!-- Dialog content -->
      <div class="text-center">
        <!-- Animated avatar icon -->
        <div class="w-20 h-20 mx-auto mb-6 rounded-2xl flex items-center justify-center animate-avatar-pulse"
             :style="{ backgroundColor: isDark ? '#3b82f6' : '#60a5fa' }">
          <IconCamera class="w-10 h-10 text-white" />
        </div>

        <!-- Animated title -->
        <h2 class="text-3xl font-bold mb-2 animate-slide-up"
            :style="{ color: isDark ? '#ffffff' : '#1a1a1a' }">
          Choose your avatar
        </h2>

        <!-- Animated file types with text reveal loop -->
        <p class="text-lg mb-8 animate-slide-up-delay flex items-center justify-center"
           :style="{ color: isDark ? '#9ca3af' : '#6b7280' }">
          Supported formats: 
          <span class="inline-block relative h-8 w-24 overflow-hidden ml-2">
            <span 
              v-for="(format, index) in fileFormats" 
              :key="format"
              class="absolute inset-0 flex items-center justify-start text-transparent bg-gradient-to-r from-red-500 via-yellow-500 via-green-500 via-blue-500 via-indigo-500 to-purple-500 bg-clip-text font-semibold transition-all duration-700 ease-in-out"
              :class="{
                'animate-text-reveal-in': currentFileFormat === index,
                'animate-text-reveal-out': currentFileFormat !== index
              }"
              :style="{
                transform: currentFileFormat === index ? 'translateY(0)' : 'translateY(100%)',
                opacity: currentFileFormat === index ? 1 : 0
              }"
            >
              {{ format }}
            </span>
          </span>
        </p>

        <!-- Action buttons -->
        <div class="space-y-3">
          <button @click="selectAvatarFile"
                  class="w-full py-4 px-6 bg-blue-600 hover:bg-blue-700 text-white rounded-2xl font-semibold transition-all duration-200 hover:scale-105 animate-slide-up-delay-2">
            Select Avatar
          </button>
          
          <button @click="closeAvatarDialog"
                  class="w-full py-3 px-6 border border-gray-300 dark:border-gray-600 rounded-2xl font-medium transition-all duration-200 hover:scale-105 animate-slide-up-delay-3"
                  :style="{ 
                    color: isDark ? '#9ca3af' : '#6b7280',
                    borderColor: isDark ? '#4b5563' : '#d1d5db'
                  }">
            Cancel
          </button>
        </div>

        <!-- Additional info -->
        <p class="text-sm mt-4 animate-slide-up-delay-4"
           :style="{ color: isDark ? '#6b7280' : '#9ca3af' }">
          Maximum file size: 5MB
        </p>
      </div>
    </div>
  </div>

  <!-- Delete Confirmation Dialog -->
  <div v-if="showDeleteDialog" 
       class="fixed inset-0 z-50 flex items-center justify-center p-4 animate-dialog-backdrop"
       @click="cancelDelete">
    <div class="fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm transition-opacity duration-300"></div>
    
    <div class="relative bg-white dark:bg-gray-800 rounded-3xl p-8 max-w-md w-full mx-4 shadow-2xl animate-dialog-content"
         :style="{ backgroundColor: isDark ? '#1a1a1a' : '#ffffff' }"
         @click.stop>
      
      <!-- Close button -->
      <button @click="cancelDelete" 
              class="absolute top-4 right-4 w-8 h-8 rounded-full flex items-center justify-center transition-colors duration-200"
              :style="{ 
                backgroundColor: isDark ? '#374151' : '#f3f4f6',
                color: isDark ? '#9ca3af' : '#6b7280'
              }">
        <IconClose class="w-5 h-5" />
      </button>

      <!-- Dialog content -->
      <div class="text-center">
        <!-- Animated warning icon -->
        <div class="w-20 h-20 mx-auto mb-6 rounded-2xl flex items-center justify-center animate-pulse"
             :style="{ backgroundColor: isDark ? '#dc2626' : '#fca5a5' }">
          <IconTrashDelete class="w-10 h-10 text-white" />
        </div>

        <!-- Title -->
        <h2 class="text-3xl font-bold mb-4"
            :style="{ color: isDark ? '#ffffff' : '#1a1a1a' }">
          Delete Upload
        </h2>

        <!-- Message -->
        <p class="text-lg mb-8"
           :style="{ color: isDark ? '#9ca3af' : '#6b7280' }">
          Are you sure you want to delete this upload? The files will be marked as deleted but remain visible in your list.
        </p>

        <!-- Action buttons -->
        <div class="space-y-3">
          <button @click="confirmDelete"
                  class="w-full py-4 px-6 bg-red-600 hover:bg-red-700 text-white rounded-2xl font-semibold transition-all duration-200 hover:scale-105">
            Yes, Delete Upload
          </button>
          
          <button @click="cancelDelete"
                  class="w-full py-3 px-6 border border-gray-300 dark:border-gray-600 rounded-2xl font-medium transition-all duration-200 hover:scale-105"
                  :style="{ 
                    color: isDark ? '#9ca3af' : '#6b7280',
                    borderColor: isDark ? '#4b5563' : '#d1d5db'
                  }">
            Cancel
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch, onBeforeUnmount } from 'vue'
import { useAuth } from '../../composables/useAuth'
import { useTheme } from '../../composables/useTheme'
import { getApiUrl, getAssetUrl } from '../../utils/apiUtils'
import UploadsTab from './components/UploadsTab.vue'
import ReverseShareTab from './components/ReverseShareTab.vue'
import StatisticsTab from './components/StatisticsTab.vue'
import UsersTab from './components/UsersTab.vue'
import SettingsTab from './components/SettingsTab.vue'

// Iconify imports
import IconUser from '~icons/heroicons/user-solid'
import IconCalendar from '~icons/heroicons/calendar-solid'
import IconShieldCheck from '~icons/heroicons/shield-check-solid'
import IconCloudArrowUp from '~icons/heroicons/cloud-arrow-up-solid'
import IconArchiveBox from '~icons/heroicons/archive-box-solid'
import IconClock from '~icons/heroicons/clock-solid'
import IconFolder from '~icons/heroicons/folder-solid'
import IconShare from '~icons/heroicons/share-solid'
import IconChartBar from '~icons/heroicons/chart-bar-solid'
import IconUsers from '~icons/heroicons/user-group-solid'
import IconCog from '~icons/heroicons/cog-6-tooth-solid'
import IconClipboardDocument from '~icons/heroicons/clipboard-document-solid'
import IconCamera from '~icons/heroicons/camera-solid'
import IconClose from '~icons/heroicons/x-mark-solid'
import IconTrashDelete from '~icons/heroicons/trash-solid'

const { user, uploads, isAuthenticated, isAdmin, isLoading, fetchCurrentUser, fetchUploads, deleteUpload } = useAuth()
const { isDark } = useTheme()

// Enhanced delete function with proper error handling
const handleDeleteUpload = async (uploadId: string) => {
  uploadToDelete.value = uploadId
  showDeleteDialog.value = true
}

const confirmDelete = async () => {
  if (!uploadToDelete.value) return

  try {
    console.log('Attempting to delete upload:', uploadToDelete.value)
    const result = await deleteUpload(uploadToDelete.value)
    
    if (result.success) {
      console.log('Upload deleted successfully')
      // Show success message
      successMessage.value = 'Upload deleted successfully!'
      showCopySuccess.value = true
      setTimeout(() => {
        showCopySuccess.value = false
        successMessage.value = 'Link copied to clipboard!'
      }, 2000)
    } else {
      console.error('Failed to delete upload:', result.message)
      alert(result.message || 'Failed to delete upload')
    }
  } catch (error) {
    console.error('Error deleting upload:', error)
    alert('Failed to delete upload. Please try again.')
  } finally {
    showDeleteDialog.value = false
    uploadToDelete.value = null
  }
}

const cancelDelete = () => {
  showDeleteDialog.value = false
  uploadToDelete.value = null
}

const activeTab = ref('uploads')
const showCreateToken = ref(false)
const reverseTokens = ref<ReverseToken[]>([])
const newToken = ref({
  name: '',
  max_uses: -1,
  expires_in: ''
})

// Success message for copy operations and other actions
const showCopySuccess = ref(false)
const successMessage = ref('Link copied to clipboard!')

// Delete confirmation dialog
const showDeleteDialog = ref(false)
const uploadToDelete = ref<string | null>(null)

// Admin functionality
const isLoadingAdminSettings = ref(true)
const adminStats = ref({
  totalUsers: 0,
  totalUploads: 0,
  totalStorage: 0
})
const quickSettings = ref({
  allowRegistration: true
})
const advancedSettings = ref({
  maxUploadSize: 100, // in MB
  maxValidity: '7days',
  theme: 'light',
  navbarTitle: 'PInGO Share'
})
const adminUsers = ref<any[]>([])
const avatarInput = ref<HTMLInputElement | null>(null)

// Expiration dropdown state
const showExpirationDropdown = ref<Record<string, boolean>>({})

interface ReverseToken {
  id: number
  token: string
  name: string
  used_count: number
  max_uses: number
  created_at: string
  expires_at: string | null
}

const activeUploads = computed(() => {
  return uploads.value.filter(upload => {
    if (!upload.expires_at) return true // Never expires
    return new Date(upload.expires_at) > new Date()
  })
})

const formatDate = (dateString: string | undefined) => {
  if (!dateString) return 'Unknown'
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  })
}

const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const formatTotalSize = () => {
  const totalBytes = uploads.value
    .filter(upload => !upload.is_deleted) // Only include non-deleted uploads
    .reduce((sum, upload) => sum + upload.total_size, 0)
  return formatFileSize(totalBytes)
}

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    successMessage.value = 'Link copied to clipboard!'
    showCopySuccess.value = true
    setTimeout(() => {
      showCopySuccess.value = false
    }, 2000)
  } catch (err) {
    console.error('Failed to copy: ', err)
    // Fallback for older browsers
    const textArea = document.createElement('textarea')
    textArea.value = text
    document.body.appendChild(textArea)
    textArea.select()
    document.body.removeChild(textArea)
    alert('Failed to copy to clipboard. Please copy manually.')
  }
}

const toggleAvailability = async (uploadId: string, currentAvailability: boolean) => {
  try {
    const response = await fetch(getApiUrl(`/uploads/${uploadId}/availability`), {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${localStorage.getItem('auth_token') || ''}`
      },
      credentials: 'include',
      body: JSON.stringify({ is_available: !currentAvailability })
    })

    if (response.ok) {
      // Refresh uploads to get updated data
      await fetchUploads()
    } else {
      throw new Error('Failed to update availability')
    }
  } catch (error) {
    console.error('Error toggling availability:', error)
    alert('Failed to update file availability')
  }
}

const fetchReverseTokens = async () => {
  try {
    const response = await fetch(getApiUrl('/reverse-tokens'), {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('auth_token') || ''}`
      },
      credentials: 'include'
    })

    if (response.ok) {
      const data = await response.json()
      reverseTokens.value = data.tokens || []
    }
  } catch (error) {
    console.error('Error fetching reverse tokens:', error)
  }
}

interface ReverseTokenData {
  name: string;
  max_uses: number;
  expires_in: string;
}

const createReverseToken = async (tokenData: ReverseTokenData) => {
  try {
    // Add validation before sending
    if (!tokenData.name || tokenData.name.trim() === '') {
      alert('Please enter a token name')
      return
    }

    // Ensure max_uses is sent as a number
    const payload = {
      name: tokenData.name.trim(),
      max_uses: parseInt(tokenData.max_uses?.toString() ?? '-1'),
      expires_in: tokenData.expires_in
    }

    console.log('Creating token with data:', payload)
    
    const response = await fetch(getApiUrl('/reverse-tokens'), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${localStorage.getItem('auth_token') || ''}`
      },
      credentials: 'include',
      body: JSON.stringify(payload)
    })

    const responseData = await response.json()
    console.log('Response:', responseData)

    if (response.ok) {
      await fetchReverseTokens()
      showCreateToken.value = false
      resetNewToken()
    } else {
      throw new Error(responseData.error || 'Failed to create token')
    }
  } catch (error) {
    console.error('Error creating reverse token:', error)
    const errorMessage = error instanceof Error ? error.message : 'Unknown error'
    alert(`Failed to create reverse share token: ${errorMessage}`)
  }
}

const deleteReverseToken = async (tokenId: number) => {
  if (!confirm('Are you sure you want to delete this token?')) return

  try {
    const response = await fetch(getApiUrl(`/reverse-tokens/${tokenId}`), {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('auth_token') || ''}`
      },
      credentials: 'include'
    })

    if (response.ok) {
      await fetchReverseTokens()
    } else {
      throw new Error('Failed to delete token')
    }
  } catch (error) {
    console.error('Error deleting reverse token:', error)
    alert('Failed to delete reverse share token')
  }
}

const resetNewToken = () => {
  newToken.value = {
    name: '',
    max_uses: -1,
    expires_in: ''
  }
}

// Admin functions
const toggleUserBlock = async (userId: number, isBlocked: boolean) => {
  try {
    const response = await fetch(getApiUrl(`/admin/users/${userId}/block`), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${localStorage.getItem('auth_token') || ''}`
      },
      body: JSON.stringify({
        blocked: isBlocked
      })
    })

    if (response.ok) {
      // Update local state
      const userIndex = adminUsers.value.findIndex(u => u.id === userId)
      if (userIndex !== -1) {
        adminUsers.value[userIndex].isBlocked = isBlocked
      }
    } else {
      console.error('Failed to toggle user block status')
    }
  } catch (error) {
    console.error('Error toggling user block:', error)
  }
}

// Change upload expiration
const changeExpiration = async (uploadId: string, validity: string) => {
  try {
    const url = getApiUrl(`/uploads/${uploadId}/expiration`)
    const token = localStorage.getItem('auth_token')
    
    console.log('=== EXPIRATION UPDATE DEBUG ===')
    console.log('Update URL:', url)
    console.log('Upload ID:', uploadId)
    console.log('Validity:', validity)
    console.log('Has token:', !!token)
    
    const response = await fetch(url, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token || ''}`
      },
      body: JSON.stringify({
        validity
      })
    })

    console.log('Expiration response status:', response.status)

    if (response.ok) {
      console.log('Expiration update successful')
      // Refresh uploads to get updated expiration dates
      await fetchUploads()
      // Close dropdown
      showExpirationDropdown.value[uploadId] = false
    } else {
      const errorText = await response.text()
      console.error('Failed to update expiration - Status:', response.status)
      console.error('Failed to update expiration - Response:', errorText)
      alert('Failed to update expiration date')
    }
  } catch (error) {
    console.error('Error updating expiration:', error)
    const errorMessage = error instanceof Error ? error.message : 'Unknown error'
    alert(`Failed to update expiration date: ${errorMessage}`)
  }
}

// Toggle expiration dropdown
const toggleExpirationDropdown = (uploadId: string) => {
  // Close all other dropdowns
  Object.keys(showExpirationDropdown.value).forEach(key => {
    if (key !== uploadId) {
      showExpirationDropdown.value[key] = false
    }
  })
  
  // Toggle the current dropdown
  showExpirationDropdown.value[uploadId] = !showExpirationDropdown.value[uploadId]
}

const fetchAdminData = async () => {
  if (!isAdmin.value) return
  
  try {
    isLoadingAdminSettings.value = true
    
    // Fetch admin statistics
    const statsResponse = await fetch(getApiUrl('/admin/stats'), {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('auth_token') || ''}`
      }
    })
    
    if (statsResponse.ok) {
      const stats = await statsResponse.json()
      adminStats.value = {
        totalUsers: Number(stats.total_users || 0),
        totalUploads: Number(stats.total_uploads || 0),
        totalStorage: Number(stats.storage_used || 0)
      }
    }

    // Fetch quick settings from main settings endpoint
    const settingsResponse = await fetch(getApiUrl('/settings'))
    if (settingsResponse.ok) {
      const settings = await settingsResponse.json()
      quickSettings.value = {
        allowRegistration: settings.allowRegistration
      }
      advancedSettings.value = {
        maxUploadSize: Math.round(settings.maxUploadSize / (1024 * 1024)), // Convert bytes to MB
        maxValidity: settings.maxValidity,
        theme: settings.theme,
        navbarTitle: settings.navbarTitle || 'PInGO Share'
      }
    }
    
    // Fetch admin users
    const usersResponse = await fetch(getApiUrl('/admin/users'), {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('auth_token') || ''}`
      }
    })
    
    if (usersResponse.ok) {
      const users = await usersResponse.json()
      adminUsers.value = users
    }
    
  } catch (error) {
    console.error('Error fetching admin data:', error)
  } finally {
    isLoadingAdminSettings.value = false
  }
}

onMounted(async () => {
  if (isAuthenticated.value) {
    await fetchCurrentUser()
    await fetchUploads()
    await fetchReverseTokens()
    if (isAdmin.value) {
      await fetchAdminData()
    }
  }
})

onBeforeUnmount(() => {
  stopFileFormatAnimation()
})

// Watch for tab changes to load admin data when needed
watch(activeTab, async (newTab: string) => {
  if ((newTab === 'statistics' || newTab === 'users' || newTab === 'settings') && isAdmin.value && adminStats.value.totalUsers === 0) {
    await fetchAdminData()
  }
})

const showAvatarDialog = ref(false)
const currentFileFormat = ref(0)
const fileFormats = ['.PNG', '.JPG', '.JPEG', '.GIF']

// Avatar functions
const openAvatarUpload = () => {
  showAvatarDialog.value = true
  startFileFormatAnimation()
}

const closeAvatarDialog = () => {
  showAvatarDialog.value = false
  stopFileFormatAnimation()
}

const selectAvatarFile = () => {
  showAvatarDialog.value = false
  stopFileFormatAnimation()
  avatarInput.value?.click()
}

// File format animation
let formatInterval: number | null = null

const startFileFormatAnimation = () => {
  currentFileFormat.value = 0
  formatInterval = setInterval(() => {
    currentFileFormat.value = (currentFileFormat.value + 1) % fileFormats.length
  }, 1200) // Change every 1.2 seconds
}

const stopFileFormatAnimation = () => {
  if (formatInterval) {
    clearInterval(formatInterval)
    formatInterval = null
  }
}

const handleAvatarError = () => {
  console.error('Failed to load avatar')
}

const handleAvatarUpload = async (event: Event) => {
  const target = event.target as HTMLInputElement
  if (!target.files || target.files.length === 0) return

  const file = target.files[0]
  
  // Validate file type - only PNG, JPG, JPEG, and GIF
  const allowedTypes = ['image/jpeg', 'image/jpg', 'image/png', 'image/gif']
  if (!allowedTypes.includes(file.type)) {
    alert('Please select a valid image file (PNG, JPG, JPEG, or GIF only)')
    return
  }

  // Validate file size (5MB max)
//   if (file.size > 5 * 1024 * 1024) {
//     alert('File size must be less than 5MB')
//     return
//   }

  const formData = new FormData()
  formData.append('avatar', file)

  try {
    const url = getApiUrl('/avatar')
    const token = localStorage.getItem('auth_token')
    
    console.log('=== AVATAR UPLOAD DEBUG ===')
    console.log('Upload URL:', url)
    console.log('Has token:', !!token)
    console.log('Token preview:', token ? token.substring(0, 20) + '...' : 'none')
    console.log('File type:', file.type)
    console.log('File size:', file.size)
    
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token || ''}`
      },
      body: formData
    })

    console.log('Response status:', response.status)
    console.log('Response headers:', Object.fromEntries(response.headers.entries()))

    if (response.ok) {
      const result = await response.json()
      console.log('Upload successful:', result)
      // Update user avatar
      if (user.value) {
        user.value.avatar = result.avatar
      }
      // Reset input
      target.value = ''
      alert('Avatar uploaded successfully!')
    } else {
      const errorText = await response.text()
      console.error('Upload failed - Status:', response.status)
      console.error('Upload failed - Response:', errorText)
      
      let errorMessage = 'Failed to upload avatar'
      try {
        const errorJson = JSON.parse(errorText)
        errorMessage = errorJson.error || errorMessage
      } catch (e) {
        errorMessage = errorText || errorMessage
      }
      
      alert(`Failed to save avatar: ${errorMessage}`)
    }
  } catch (error) {
    console.error('Error uploading avatar:', error)
    const errorMessage = error instanceof Error ? error.message : 'Unknown error'
    alert(`Upload failed. Please try again. Error: ${errorMessage}`)
  }
}

</script>

<style scoped>
/* Dialog animations */
@keyframes dialog-backdrop {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes dialog-content {
  from { 
    opacity: 0; 
    transform: scale(0.95) translateY(-20px); 
  }
  to { 
    opacity: 1; 
    transform: scale(1) translateY(0); 
  }
}

@keyframes avatar-pulse {
  0%, 100% { 
    transform: scale(1); 
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.4);
  }
  50% { 
    transform: scale(1.05); 
    box-shadow: 0 0 0 20px rgba(59, 130, 246, 0);
  }
}

@keyframes slide-up {
  from { 
    opacity: 0; 
    transform: translateY(20px); 
  }
  to { 
    opacity: 1; 
    transform: translateY(0); 
  }
}

@keyframes rainbow {
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}

@keyframes text-reveal-in {
  0% { 
    transform: translateY(100%); 
    opacity: 0; 
  }
  100% { 
    transform: translateY(0); 
    opacity: 1; 
  }
}

@keyframes text-reveal-out {
  0% { 
    transform: translateY(0); 
    opacity: 1; 
  }
  100% { 
    transform: translateY(-100%); 
    opacity: 0; 
  }
}

/* Animation classes */
.animate-dialog-backdrop {
  animation: dialog-backdrop 0.3s ease-out;
}

.animate-dialog-content {
  animation: dialog-content 0.4s ease-out;
}

.animate-avatar-pulse {
  animation: avatar-pulse 2s ease-in-out infinite;
}

.animate-slide-up {
  animation: slide-up 0.6s ease-out;
}

.animate-slide-up-delay {
  animation: slide-up 0.6s ease-out 0.2s both;
}

.animate-slide-up-delay-2 {
  animation: slide-up 0.6s ease-out 0.4s both;
}

.animate-slide-up-delay-3 {
  animation: slide-up 0.6s ease-out 0.6s both;
}

.animate-slide-up-delay-4 {
  animation: slide-up 0.6s ease-out 0.8s both;
}

.animate-rainbow {
  background-size: 400% 400%;
  animation: rainbow 3s ease-in-out infinite;
}

.animate-text-reveal-in {
  animation: text-reveal-in 0.7s ease-out forwards;
}

.animate-text-reveal-out {
  animation: text-reveal-out 0.7s ease-out forwards;
}

/* Apple-style backdrop blur */
.backdrop-blur-sm {
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
}

/* Hide scrollbar for tab navigation */
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
</style>