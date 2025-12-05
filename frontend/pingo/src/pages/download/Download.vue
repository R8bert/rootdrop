<template>
  <div class="min-h-screen flex relative overflow-hidden bg-white dark:bg-black text-neutral-900 dark:text-white">
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

    <!-- Main Content -->
    <div class="w-full flex items-center justify-center p-8 lg:p-12">
      
      <!-- Download -->
      <div class="w-full max-w-4xl p-8 rounded-3xl shadow-xl border border-white/10 relative overflow-hidden"
          :class="isDark ? 'bg-neutral-900/95' : 'bg-white/95'">
          
        <div class="relative z-10">
        
          <!-- Compact Logo/Brand -->
          <div class="mb-8 animate-fade-in">
            <h2 class="text-lg font-semibold tracking-tight transition-colors duration-300" 
                :class="isDark ? 'text-white' : 'text-neutral-900'">
              File sharing 
            </h2>
            <p class="text-xs mt-1 transition-colors duration-300" :class="isDark ? 'text-neutral-500' : 'text-neutral-500'">
              by rootdrop
            </p>
          </div>

          <!-- Enhanced Loading State -->
          <div v-if="loading" class="relative text-center space-y-10 animate-fade-in">
            <!-- Modern loading spinner -->
            <div class="relative mx-auto w-32 h-32">
              <div class="absolute inset-0 rounded-full bg-gradient-to-r from-neutral-700 via-neutral-600 to-neutral-700 p-1 animate-spin">
                <div class="w-full h-full rounded-full bg-white dark:bg-slate-900"></div>
              </div>
              <div class="absolute inset-4 rounded-full bg-gradient-to-r from-neutral-600 to-neutral-500 animate-spin-reverse">
                <div class="w-full h-full rounded-full bg-white dark:bg-slate-900"></div>
              </div>
              <div class="absolute inset-8 rounded-full bg-gradient-to-r from-neutral-500 to-neutral-400 animate-pulse">
                <div class="w-full h-full rounded-full bg-white dark:bg-slate-900 flex items-center justify-center">
                  <div class="w-3 h-3 bg-gradient-to-r from-neutral-600 to-neutral-500 rounded-full animate-bounce"></div>
                </div>
              </div>
            </div>

            <div class="space-y-4">
              <h1 class="text-4xl lg:text-5xl font-bold transition-colors duration-300"
                  :class="isDark ? 'text-white' : 'text-neutral-900'">
                Preparing your files
              </h1>
              <p class="text-xl text-slate-600 dark:text-slate-400 animate-fade-in" style="animation-delay: 0.3s;">
                Setting up your secure download experience
              </p>
              <!-- Progress indicator -->
              <div class="flex justify-center mt-6">
                <div class="flex space-x-2">
                  <div class="w-3 h-3 bg-neutral-600 rounded-full animate-bounce" style="animation-delay: 0s"></div>
                  <div class="w-3 h-3 bg-neutral-500 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                  <div class="w-3 h-3 bg-neutral-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                </div>
              </div>
            </div>
          </div>

          <!-- Enhanced Content State -->
          <div v-else-if="files.length > 0" class="space-y-8">

            <!-- Modern Header Section -->
            <div class="text-center space-y-6 animate-fade-in" style="animation-delay: 0.1s;">
              <!-- Status badge -->
              <div class="inline-flex items-center gap-3 px-6 py-3 rounded-full border backdrop-blur-sm"
                   :class="isDark ? 'bg-neutral-700/30 border-neutral-600/40' : 'bg-neutral-200/50 border-neutral-300/40'">
                <div class="w-3 h-3 bg-neutral-500 rounded-full animate-pulse"></div>
                <span class="text-sm font-semibold"
                      :class="isDark ? 'text-neutral-300' : 'text-neutral-700'">
                  {{ files.length }} file{{ files.length > 1 ? 's' : '' }} ready
                </span>
              </div>

              <!-- Main title -->
              <div class="space-y-2">
                <h1 class="text-3xl lg:text-4xl font-bold leading-tight transition-colors duration-300"
                    :class="isDark ? 'text-white' : 'text-neutral-900'">
                  Download Files
                </h1>
                <p class="text-base text-slate-600 dark:text-slate-400">
                  Secure and instant
                </p>
              </div>
            </div>

            <!-- Uploader Info Card -->
            <div v-if="uploader" class="animate-fade-in p-6 rounded-2xl" style="animation-delay: 0.2s;"
                :class="isDark ? 'bg-neutral-800/50' : 'bg-neutral-100'">
              <div class="flex flex-col lg:flex-row lg:items-center gap-6">
                
                <!-- Avatar - Enhanced Size -->
                <div class="flex items-center gap-6">
                  <div class="relative group">
                    <div class="w-24 h-24 rounded-2xl overflow-hidden backdrop-blur-sm border-4 border-white/50 dark:border-slate-600/50 shadow-2xl transition-all duration-300 group-hover:scale-105 group-hover:border-blue-500/50">
                      <img v-if="uploader.avatar"
                           :src="getAssetUrl(uploader.avatar)"
                           :alt="uploader.username"
                           class="w-full h-full object-cover"
                           @error="handleAvatarError" />
                      <div v-else class="w-full h-full bg-gradient-to-br from-neutral-700 to-neutral-800 flex items-center justify-center">
                        <span class="text-white text-4xl font-bold">{{ uploader.username.charAt(0).toUpperCase() }}</span>
                      </div>
                    </div>
                    <!-- Decorative ring -->
                    <div class="absolute inset-0 rounded-2xl bg-neutral-500 opacity-0 group-hover:opacity-20 blur-xl transition-opacity duration-300 -z-10"></div>
                  </div>

                  <!-- User Info -->
                  <div>
                    <h3 class="text-2xl font-bold transition-colors duration-300"
                        :class="isDark ? 'text-white' : 'text-neutral-900'">{{ uploader.username }}</h3>
                    <p class="text-sm text-slate-600 dark:text-slate-400 mt-1">{{ uploader.email }}</p>
                  </div>
                </div>

                <!-- Stats -->
                <div class="flex-1 grid grid-cols-2 lg:grid-cols-3 gap-3">
                  <div class="text-center p-3 rounded-xl" :class="isDark ? 'bg-neutral-700/50' : 'bg-neutral-200'">
                    <div class="text-lg font-bold"
                         :class="isDark ? 'text-neutral-300' : 'text-neutral-700'">{{ files.length }}</div>
                    <div class="text-xs text-slate-600 dark:text-slate-400">Files</div>
                  </div>
                  
                  <div class="text-center p-3 rounded-xl" :class="isDark ? 'bg-neutral-700/50' : 'bg-neutral-200'">
                    <div class="text-lg font-bold"
                         :class="isDark ? 'text-neutral-300' : 'text-neutral-700'">{{ formatTotalSize() }}</div>
                    <div class="text-xs text-slate-600 dark:text-slate-400">Size</div>
                  </div>
                  
                  <div v-if="uploader?.expirationDate" class="text-center p-3 rounded-xl" :class="isDark ? 'bg-neutral-700/50' : 'bg-neutral-200'">
                    <div class="text-sm font-bold" :class="formatExpirationDate(uploader.expirationDate).isExpired ? 'text-red-500' : isDark ? 'text-neutral-300' : 'text-neutral-700'">
                      {{ formatExpirationDate(uploader.expirationDate).timeLeft }}
                    </div>
                    <div class="text-xs text-slate-600 dark:text-slate-400">Expires</div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Download All Button -->
            <div class="text-center animate-fade-in" style="animation-delay: 0.3s;">
              <button
                @click="downloadAll"
                :disabled="downloadingAll"
                class="px-8 py-3 bg-gradient-to-r from-neutral-800 to-neutral-700 text-white font-semibold rounded-lg transition-all duration-300 hover:scale-105 disabled:opacity-50 shadow-lg"
              >
                <span v-if="downloadingAll" class="flex items-center justify-center gap-3">
                  <div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                  <span>Downloading All...</span>
                </span>
                <span v-else class="flex items-center gap-3">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                  </svg>
                  <span>Download All Files</span>
                  <span class="text-sm opacity-80">({{ formatTotalSize() }})</span>
                </span>
              </button>
            </div>

            <!-- Files List -->
            <div class="space-y-3 animate-fade-in" style="animation-delay: 0.4s;">
              <div class="text-center mb-4">
                <h2 class="text-xl font-bold">Files ({{ files.length }})</h2>
              </div>

              <div class="space-y-3">
                <div v-for="(file, index) in files" :key="'file-' + index + '-' + file.name"
                     class="group flex flex-col rounded-xl transition-all duration-300 hover:scale-[1.02]"
                     :class="isDark ? 'bg-neutral-800/50 hover:bg-neutral-800/70' : 'bg-neutral-100 hover:bg-neutral-200'">

                  <!-- File info and buttons row -->
                  <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4 p-4">
                    <!-- File Icon and Info -->
                    <div class="flex items-center gap-3 flex-1 min-w-0">
                      <!-- File icon -->
                      <div class="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0"
                          :class="isDark ? 'bg-neutral-700' : 'bg-neutral-300'">
                        <img
                          :src="getFileIconPath(file.name)"
                          :alt="getFileIconAltText(file.name)"
                          class="w-6 h-6 object-contain"
                        />
                      </div>

                      <!-- File details -->
                      <div class="flex-1 min-w-0">
                        <h3 class="text-base font-bold truncate" :title="file.name">
                          {{ file.name }}
                        </h3>
                        <div class="flex items-center gap-2 text-sm mt-1">
                          <span class="text-slate-600 dark:text-slate-400">
                            {{ formatFileSize(file.size) }}
                          </span>
                          <span class="px-2 py-0.5 rounded-full text-xs font-medium"
                              :class="isDark ? 'bg-neutral-700' : 'bg-neutral-200'">
                            {{ getFileExtension(file.name).toUpperCase() }}
                          </span>
                        </div>
                      </div>
                    </div>

                    <!-- Action Buttons -->
                    <div class="flex flex-wrap items-center gap-2 sm:flex-nowrap">
                      <!-- Preview Button (for images and PDFs) -->
                      <button v-if="canPreview(file.name)"
                              @click="togglePreview(file.name)"
                              class="px-4 py-2 rounded-lg font-semibold transition-all duration-200 hover:scale-105 shadow-md text-sm"
                              :class="isDark ? 'bg-neutral-700 hover:bg-neutral-600 text-white' : 'bg-neutral-300 hover:bg-neutral-400 text-neutral-900'">
                        <span class="flex items-center gap-2">
                          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                            <path stroke-linecap="round" stroke-liejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
                          </svg>
                          <span>Preview</span>
                        </span>
                      </button>
                      
                      <!-- Download Button -->
                      <button @click="downloadFile(file, index)"
                              :disabled="downloadingStates[index]"
                              class="px-4 py-2 bg-gradient-to-r from-neutral-800 to-neutral-700 text-white font-semibold rounded-lg transition-all duration-200 hover:scale-105 disabled:opacity-50 shadow-md text-sm">
                        <span v-if="downloadingStates[index]" class="flex items-center gap-2">
                          <div class="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                          <span>...</span>
                        </span>
                        <span v-else class="flex items-center gap-2">
                          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                          </svg>
                          <span>Download</span>
                        </span>
                      </button>
                    </div>
                  </div>

                  <!-- Preview Section -->
                  <div v-if="previewingFiles[file.name]" 
                       class="px-4 pb-4">
                    <div class="p-4 rounded-xl overflow-hidden"
                         :class="isDark ? 'bg-neutral-900/50' : 'bg-neutral-200/50'">
                      <div class="relative">
                        <img v-if="isImage(file.name)"
                             :src="getFilePreviewUrl(file.name)"
                             :alt="file.name"
                             class="max-w-full max-h-96 mx-auto rounded-lg shadow-lg" />
                        <iframe v-else-if="isPDF(file.name)"
                                :src="getFilePreviewUrl(file.name)"
                                class="w-full h-96 rounded-lg shadow-lg border-0"></iframe>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

          </div>

          <!-- Error State -->
          <div v-else class="text-center space-y-6 animate-fade-in">
            <div class="w-20 h-20 mx-auto rounded-full flex items-center justify-center"
                :class="isDark ? 'bg-red-500/20' : 'bg-red-100'">
              <svg class="w-10 h-10 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
              </svg>
            </div>

            <div>
              <h1 class="text-3xl font-bold bg-gradient-to-r from-red-600 to-pink-600 bg-clip-text text-transparent mb-2">
                Share Not Found
              </h1>
              <p class="text-base text-slate-600 dark:text-slate-400">
                This share may have expired or been removed
              </p>
            </div>

            <div class="flex gap-3 justify-center">
              <button
                @click="$router.push('/')"
                class="px-6 py-3 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-semibold rounded-lg transition-all duration-300 hover:scale-105"
              >
                <span class="flex items-center gap-2">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
                  </svg>
                  Go Home
                </span>
              </button>

              <button
                @click="$router.go(-1)"
                class="px-6 py-3 rounded-lg font-medium transition-all duration-300 hover:scale-105"
                :class="isDark ? 'bg-neutral-700 hover:bg-neutral-600 text-white' : 'bg-neutral-200 hover:bg-neutral-300 text-neutral-900'"
              >
                <span class="flex items-center gap-2">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                  </svg>
                  Go Back
                </span>
              </button>
            </div>
          </div>

        </div>
      </div>
    </div>

    <!-- GitHub Icon -->
    <a 
      href="https://github.com/R8bert/PInGO-Share" 
      target="_blank" 
      rel="noopener noreferrer" 
      class="fixed bottom-4 left-4 z-40 p-2 rounded-lg backdrop-blur-sm transition-all duration-300 hover:scale-140"
      :class="isDark ? 'text-gray-400 hover:text-white bg-white/5' : 'text-gray-600 hover:text-gray-900 bg-white/50'"
      title="View GitHub Repository"
    >
      <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
      </svg>
    </a>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useTheme } from '../../composables/useTheme'
import { useIcons } from '../../composables/useIcons'
import { getApiUrl, getAssetUrl } from '../../utils/apiUtils'
import Galaxy from '../../blocks/Backgrounds/Galaxy/Galaxy.vue'
import axios from 'axios'

// Interfaces
interface FileInfo {
  name: string
  size: number
}

interface UploaderInfo {
  username: string
  email: string
  avatar?: string
  expirationDate?: string
}

interface Message {
  text: string
  type: 'success' | 'error'
}

const { isDark } = useTheme()
const { 
  getFileIcon, 
  getFileIconAlt,
  getFileExtension 
} = useIcons()
const route = useRoute()

// Galaxy loading state
const galaxyLoaded = ref(false)

// Helper methods for file icons
const getFileIconPath = (filename: string) => {
  return getFileIcon(filename)
}

const getFileIconAltText = (filename: string) => {
  return getFileIconAlt(filename)
}

// State
const loading = ref(true)
const files = ref<FileInfo[]>([])
const uploader = ref<UploaderInfo | null>(null)
const downloadingAll = ref(false)
const downloadingStates = ref<Record<number, boolean>>({})
const message = ref<Message | null>(null)
const previewingFiles = ref<Record<string, boolean>>({})

// Computed properties for API URLs
const getFileDownloadUrl = (fileName: string) => {
  return getApiUrl(`/file/${route.params.id}/${fileName}`)
}

const getFilePreviewUrl = (fileName: string) => {
  return getApiUrl(`/file/${route.params.id}/${fileName}`)
}

const getDownloadAllUrl = () => {
  return getApiUrl(`/download/${route.params.id}`)
}

// Preview helper functions
const isImage = (fileName: string): boolean => {
  const ext = fileName.split('.').pop()?.toLowerCase()
  return ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'bmp'].includes(ext || '')
}

const isPDF = (fileName: string): boolean => {
  return fileName.toLowerCase().endsWith('.pdf')
}

const canPreview = (fileName: string): boolean => {
  return isImage(fileName) || isPDF(fileName)
}

const togglePreview = (fileName: string) => {
  previewingFiles.value[fileName] = !previewingFiles.value[fileName]
}

// Methods
const fetchFiles = async () => {
  try {
    const response = await axios.get(getApiUrl(`files/${route.params.id}`))
    files.value = response.data.files || []
    uploader.value = response.data.uploader || null
    loading.value = false
  } catch (error) {
    console.error('Error fetching files:', error)
    message.value = { text: 'Share not found', type: 'error' }
    loading.value = false
  }
}

const downloadFile = async (file: FileInfo, index: number) => {
  downloadingStates.value[index] = true
  
  try {
    const response = await axios.get(
      getFileDownloadUrl(file.name),
      { responseType: 'blob' }
    )
    
    const url = window.URL.createObjectURL(new Blob([response.data]))
    const link = document.createElement('a')
    link.href = url
    link.setAttribute('download', file.name)
    document.body.appendChild(link)
    link.click()
    link.remove()
    window.URL.revokeObjectURL(url)
    
    showMessage('File downloaded successfully!', 'success')
  } catch (error) {
    console.error('Download error:', error)
    showMessage('Download failed. Please try again.', 'error')
  } finally {
    downloadingStates.value[index] = false
  }
}

const downloadAll = async () => {
  downloadingAll.value = true
  
  try {
    const response = await axios.get(
      getDownloadAllUrl(),
      { responseType: 'blob' }
    )
    
    const url = window.URL.createObjectURL(new Blob([response.data]))
    const link = document.createElement('a')
    link.href = url
    link.setAttribute('download', `share-${route.params.id}.zip`)
    document.body.appendChild(link)
    link.click()
    link.remove()
    window.URL.revokeObjectURL(url)
    
    showMessage('All files downloaded successfully!', 'success')
  } catch (error) {
    console.error('Download all error:', error)
    showMessage('Download failed. Please try again.', 'error')
  } finally {
    downloadingAll.value = false
  }
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const formatTotalSize = (): string => {
  const total = files.value.reduce((sum, file) => sum + file.size, 0)
  return formatFileSize(total)
}

const formatExpirationDate = (dateString: string) => {
  const date = new Date(dateString)
  const now = new Date()
  const isExpired = date < now
  
  if (isExpired) {
    return { text: 'Expired', isExpired: true, timeLeft: 'Expired' }
  }
  
  const diffTime = date.getTime() - now.getTime()
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
  const diffHours = Math.ceil(diffTime / (1000 * 60 * 60))
  const diffMinutes = Math.ceil(diffTime / (1000 * 60))
  
  let timeLeft = ''
  if (diffDays > 1) {
    timeLeft = `${diffDays}d`
  } else if (diffHours > 1) {
    timeLeft = `${diffHours}h`
  } else if (diffMinutes > 1) {
    timeLeft = `${diffMinutes}m`
  } else {
    timeLeft = '<1m'
  }
  
  if (diffDays === 1) return { text: 'Expires today', isExpired: false, timeLeft }
  if (diffDays <= 7) return { text: `Expires in ${diffDays} days`, isExpired: false, timeLeft }
  
  return { text: `Expires ${date.toLocaleDateString()}`, isExpired: false, timeLeft }
}

const handleAvatarError = () => {
  if (uploader.value) {
    uploader.value.avatar = undefined
  }
}

const showMessage = (text: string, type: 'success' | 'error') => {
  message.value = { text, type }
  setTimeout(() => {
    message.value = null
  }, 3000)
}

// Initialize
onMounted(() => {
  // Delay galaxy loading to prevent freeze
  setTimeout(() => {
    galaxyLoaded.value = true
  }, 100)
  
  fetchFiles()
})
</script>

<style scoped>
/* Enhanced animations for modern design */
.animate-fade-in {
  animation: fadeIn 0.8s ease-out forwards;
  opacity: 0;
  transform: translateY(30px);
}

@keyframes fadeIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in-word {
  animation: fadeInWord 0.6s ease-out forwards;
  opacity: 0;
  transform: translateY(20px);
}

@keyframes fadeInWord {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Gradient text animation */
.animate-gradient-x {
  background-size: 200% 200%;
  animation: gradientX 4s ease infinite;
}

@keyframes gradientX {
  0%, 100% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
}

/* Floating animations for background orbs */
.animate-float {
  animation: float 6s ease-in-out infinite;
}

.animate-float-slow {
  animation: float 8s ease-in-out infinite;
}

.animate-float-slower {
  animation: float 10s ease-in-out infinite;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px) rotate(0deg);
  }
  33% {
    transform: translateY(-20px) rotate(120deg);
  }
  66% {
    transform: translateY(10px) rotate(240deg);
  }
}

/* Gradient shift animation for background */
.animate-gradient-shift {
  background-size: 400% 400%;
  animation: gradientShift 15s ease infinite;
}

@keyframes gradientShift {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

/* Custom spinner animations */
.animate-spin-reverse {
  animation: spin 2s linear infinite reverse;
}

.animate-spin-slow {
  animation: spin 4s linear infinite;
}

/* Pulse glow effect */
.pulse-glow {
  animation: pulseGlow 3s ease-in-out infinite;
}

@keyframes pulseGlow {
  0%, 100% {
    box-shadow: 0 0 20px rgba(59, 130, 246, 0.3);
  }
  50% {
    box-shadow: 0 0 40px rgba(59, 130, 246, 0.6), 0 0 60px rgba(147, 51, 234, 0.3);
  }
}

/* Hover effects */
.hover-lift {
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.hover-lift:hover {
  transform: translateY(-4px);
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
}

/* Glass morphism effect */
.glass {
  backdrop-filter: blur(16px) saturate(180%);
  background-color: rgba(255, 255, 255, 0.75);
  border: 1px solid rgba(209, 213, 219, 0.3);
}

.glass-dark {
  backdrop-filter: blur(16px) saturate(180%);
  background-color: rgba(17, 24, 39, 0.75);
  border: 1px solid rgba(55, 65, 81, 0.3);
}
</style>
