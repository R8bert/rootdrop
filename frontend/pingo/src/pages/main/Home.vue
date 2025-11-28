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

        <!-- Left Side - Form Container -->
        <div class="w-full lg:flex-1 flex items-center justify-center p-8 lg:p-12">
            
            <!-- Form Card -->
            <div class="w-full max-w-xl p-8 rounded-3xl shadow-xl border border-white/10 relative overflow-hidden"
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

                <!-- Request Files Header -->
                <div class="mb-6 text-center animate-fade-in" style="animation-delay: 0.1s">
                    <h3 class="text-sm font-medium mb-1 transition-colors duration-300" :class="isDark ? 'text-white' : 'text-neutral-900'">
                        Request files
                    </h3>
                </div>

                <!-- Upload Section -->
                <div v-if="!uploadComplete" class="space-y-4">
                    <!-- Add Files Buttons -->
                    <div class="space-y-3">
                            <button @click="triggerFileInput"
                                class="w-full px-4 py-3 rounded-lg font-medium text-white bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 text-sm flex items-center justify-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                                </svg>
                                <span>Add files</span>
                            </button>

                            <button @click="triggerFolderInput"
                                class="w-full px-4 py-3 rounded-lg font-medium text-white border border-neutral-700 hover:bg-neutral-800 text-sm flex items-center justify-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
                                </svg>
                                <span>Add folder</span>
                            </button>
                        </div>

                        <!-- Hidden File Inputs -->
                        <input ref="fileInput" type="file" multiple @change="onFileChange" class="hidden" />
                        <input ref="folderInput" type="file" webkitdirectory @change="onFolderChange" class="hidden" />

                        <!-- Drag and Drop Zone -->
                        <div v-if="selectedFiles.length === 0"
                            @drop.prevent="onDrop"
                            @dragover.prevent="isDragging = true"
                            @dragleave.prevent="isDragging = false"
                            class="mt-4 p-8 rounded-lg border border-dashed cursor-pointer transition-colors duration-200"
                            :class="[
                                isDragging 
                                    ? 'border-blue-500 bg-blue-500/10' 
                                    : 'border-neutral-700 hover:border-neutral-600 hover:bg-neutral-800/30'
                            ]">
                            
                            <div class="text-center">
                                <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-gradient-to-br from-neutral-700 to-neutral-600 flex items-center justify-center"
                                     :class="isDragging ? 'bg-gradient-to-br from-blue-500 to-purple-600' : ''">
                                    <svg class="w-8 h-8 text-neutral-400" 
                                         :class="isDragging ? 'text-white' : ''" 
                                         fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"/>
                                    </svg>
                                </div>
                                
                                <p class="text-sm text-neutral-400"
                                   :class="isDragging ? 'text-blue-300 font-semibold' : ''">
                                    {{ isDragging ? 'Drop files here!' : 'or drop files here' }}
                                </p>
                            </div>
                        </div>

                        <!-- File List with enhanced styling -->
                        <div v-if="selectedFiles.length > 0" class="mt-6 space-y-2 animate-slide-up" style="animation-delay: 0.2s">
                            <!-- Total size indicator -->
                            <div class="flex items-center justify-between mb-3 p-3 rounded-lg backdrop-blur-sm transition-all duration-300 hover:scale-[1.01]" 
                                :class="isDark ? 'bg-neutral-800/30' : 'bg-neutral-100'">
                                <span class="text-sm font-medium">{{ selectedFiles.length }} file{{ selectedFiles.length > 1 ? 's' : '' }}</span>
                                <span class="text-sm transition-colors duration-300" :class="isDark ? 'text-neutral-400' : 'text-neutral-600'">
                                    {{ formatFileSize(selectedFiles.reduce((sum, f) => sum + f.size, 0)) }} total
                                </span>
                            </div>

                            <!-- Tree View for Folders -->
                            <div v-if="isFolderMode && folderTree" class="space-y-1">
                                <TreeNode :node="folderTree" :level="0" />
                            </div>

                            <!-- Flat List for Individual Files -->
                            <div v-else class="space-y-2">
                                <div v-for="(file, index) in selectedFiles" :key="index"
                                    class="group flex items-center gap-3 p-3 rounded-xl transition-all duration-300 hover:scale-[1.02] hover:shadow-lg cursor-pointer"
                                    :class="isDark ? 'bg-neutral-800/50 hover:bg-neutral-800/70' : 'bg-neutral-100 hover:bg-neutral-200'">
                                    
                                    <!-- File icon with animation -->
                                    <div class="relative">
                                        <div class="w-12 h-12 rounded-xl flex items-center justify-center text-xs font-bold uppercase flex-shrink-0 transition-all duration-300 group-hover:scale-110 group-hover:rotate-3"
                                            :class="isDark ? 'bg-gradient-to-br from-neutral-700 to-neutral-600' : 'bg-gradient-to-br from-neutral-300 to-neutral-400'">
                                            {{ getFileExtension(file).slice(0, 3) }}
                                        </div>
                                        <div class="absolute -bottom-1 -right-1 w-5 h-5 rounded-full flex items-center justify-center shadow-lg transition-all duration-300 group-hover:scale-110"
                                            :class="getFileTypeColor(file)">
                                            <svg class="w-3 h-3 text-white drop-shadow-sm transition-transform duration-300 group-hover:rotate-12" fill="currentColor" viewBox="0 0 20 20">
                                                <path d="M4 3a2 2 0 100 4h12a2 2 0 100-4H4z"></path>
                                                <path fill-rule="evenodd" d="M3 8h14v7a2 2 0 01-2 2H5a2 2 0 01-2-2V8zm5 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                                            </svg>
                                        </div>
                                    </div>
                                    
                                    <!-- File info with enhanced interactions -->
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm font-semibold truncate transition-colors duration-300 group-hover:text-blue-600 dark:group-hover:text-blue-400">{{ file.name }}</p>
                                        <div class="flex items-center gap-2 mt-1">
                                            <p class="text-xs transition-colors duration-300" :class="isDark ? 'text-neutral-500 group-hover:text-neutral-400' : 'text-neutral-600 group-hover:text-neutral-700'">
                                                {{ formatFileSize(file.size) }}
                                            </p>
                                            <span class="text-xs px-2 py-0.5 rounded-full font-medium transition-all duration-300 group-hover:scale-105"
                                                :class="isDark ? 'bg-neutral-700 text-neutral-300 group-hover:bg-neutral-600' : 'bg-neutral-200 text-neutral-700 group-hover:bg-neutral-300'">
                                                {{ getFileExtension(file).toUpperCase() }}
                                            </span>
                                        </div>
                                    </div>
                                    
                                    <!-- Remove button with enhanced animation -->
                                    <button @click.stop="removeFile(index)" 
                                        class="w-9 h-9 rounded-lg flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-300 hover:scale-110 hover:rotate-90"
                                        :class="isDark ? 'hover:bg-red-500/20 text-neutral-400 hover:text-red-400' : 'hover:bg-red-500/10 text-neutral-600 hover:text-red-600'">
                                        <IconXMark class="w-5 h-5 transition-transform duration-200" />
                                    </button>
                                </div>
                            </div>
                        </div>

                        <!-- Options - Compact Style -->
                        <div v-if="selectedFiles.length > 0" class="mt-6 space-y-3">
                            <!-- Expiry select -->
                            <div class="flex items-center gap-2 group">
                                <svg class="w-4 h-4 text-neutral-500 transition-all duration-300 group-hover:text-neutral-400 group-hover:rotate-180" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                <select v-model="selectedValidity" 
                                    class="flex-1 px-3 py-2 rounded-lg border bg-transparent cursor-pointer text-sm text-white outline-none focus:ring-2 focus:ring-blue-500/50 transition-all duration-300 hover:border-neutral-600"
                                    :class="'border-neutral-700'">
                                    <option v-for="option in validityOptions" :key="option.value" :value="option.value"
                                        class="bg-neutral-900">
                                        {{ option.label }}
                                    </option>
                                </select>
                            </div>

                            <!-- Transfer Button -->
                            <button @click="uploadFiles" :disabled="isUploading"
                                class="w-full py-3 rounded-lg font-medium text-white mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                                :class="isUploading ? 'bg-blue-500 cursor-not-allowed' : 'bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800'">
                                
                                <span class="flex items-center justify-center gap-2">
                                    <svg v-if="isUploading" class="w-4 h-4 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                                    </svg>
                                    <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path>
                                    </svg>
                                    {{ isUploading ? 'Transferring...' : 'Transfer' }}
                                </span>
                            </button>

                            <!-- Progress with enhanced styling -->
                            <transition
                                enter-active-class="transition-all duration-300"
                                enter-from-class="opacity-0 translate-y-4"
                                enter-to-class="opacity-100 translate-y-0">
                                <div v-if="isUploading" class="space-y-3 p-4 rounded-xl" 
                                    :class="isDark ? 'bg-neutral-800/50' : 'bg-neutral-100'">
                                    <div class="flex justify-between text-sm font-medium">
                                        <span class="flex items-center gap-2">
                                            <span class="w-2 h-2 rounded-full bg-blue-500 animate-pulse"></span>
                                            Uploading
                                        </span>
                                        <span class="text-blue-600">{{ progress }}%</span>
                                    </div>
                                    <div class="relative h-2 rounded-full overflow-hidden" :class="isDark ? 'bg-neutral-700' : 'bg-neutral-200'">
                                        <div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-blue-600 transition-all duration-300 rounded-full" 
                                            :style="{ width: `${progress}%` }">
                                            <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/30 to-transparent animate-shimmer"></div>
                                        </div>
                                    </div>
                                    <div class="flex justify-between text-xs" :class="isDark ? 'text-neutral-400' : 'text-neutral-600'">
                                        <span v-if="uploadSpeed > 0">{{ formatFileSize(uploadSpeed) }}/s</span>
                                        <span v-if="estimatedTimeRemaining > 0">
                                            {{ Math.ceil(estimatedTimeRemaining / 60) }} min remaining
                                        </span>
                                    </div>
                                </div>
                            </transition>
                        </div>
                    </div>

                    <!-- Success State with enhanced styling -->
                    <transition
                        enter-active-class="transition-all duration-500"
                        enter-from-class="opacity-0 scale-95"
                        enter-to-class="opacity-100 scale-100">
                        <div v-if="uploadComplete" class="py-12 text-center space-y-6">
                            <!-- Success Icon with animation -->
                            <div class="inline-flex">
                                <div class="relative">
                                    <div class="w-20 h-20 rounded-full bg-gradient-to-br from-green-500 to-green-600 flex items-center justify-center shadow-xl animate-bounce-once">
                                        <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                    </div>
                                    <div class="absolute inset-0 rounded-full bg-green-500 animate-ping opacity-75"></div>
                                </div>
                            </div>
                            
                            <div>
                                <h3 class="text-2xl font-bold mb-2">Transfer complete!</h3>
                                <p class="text-base" :class="isDark ? 'text-neutral-400' : 'text-neutral-600'">
                                    Your files are ready to share
                                </p>
                            </div>

                            <div class="space-y-3 max-w-md mx-auto">
                                <div class="group p-4 rounded-xl border-2 flex items-center gap-3 transition-all duration-200 hover:shadow-lg"
                                    :class="isDark ? 'bg-neutral-800/50 border-neutral-700 hover:border-neutral-600' : 'bg-neutral-100 border-neutral-300 hover:border-neutral-400'">
                                    <input :value="uploadLink" readonly 
                                        class="flex-1 bg-transparent outline-none text-sm font-mono truncate" />
                                    <button @click="copyLink"
                                        class="px-5 py-2.5 rounded-lg text-sm font-semibold transition-all duration-200"
                                        :class="copied 
                                            ? 'bg-green-500 text-white' 
                                            : isDark ? 'bg-blue-600 text-white hover:bg-blue-700' : 'bg-blue-600 text-white hover:bg-blue-700'">
                                        <span class="flex items-center gap-2">
                                            <svg v-if="copied" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                            </svg>
                                            <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                            </svg>
                                            {{ copied ? 'Copied!' : 'Copy link' }}
                                        </span>
                                    </button>
                                </div>

                                <button @click="uploadNew"
                                    class="w-full py-3.5 rounded-xl font-semibold transition-all duration-200 flex items-center justify-center gap-2"
                                    :class="isDark ? 'bg-neutral-800 hover:bg-neutral-700 text-white' : 'bg-neutral-200 hover:bg-neutral-300 text-neutral-900'">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                                    </svg>
                                    Transfer new files
                                </button>
                            </div>
                        </div>
                    </transition>
                
                <!-- Floating Action Button -->
                <transition
                    enter-active-class="transition-all duration-300"
                    enter-from-class="opacity-0 scale-75 translate-y-4"
                    enter-to-class="opacity-100 scale-100 translate-y-0"
                    leave-active-class="transition-all duration-200"
                    leave-from-class="opacity-100 scale-100 translate-y-0"
                    leave-to-class="opacity-0 scale-75 translate-y-4">
                    <button v-if="selectedFiles.length > 0 && !uploadComplete"
                            @click="uploadFiles"
                            :disabled="isUploading"
                            class="fixed bottom-6 right-6 w-16 h-16 rounded-full bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-2xl hover:shadow-blue-500/25 transition-all duration-300 hover:scale-110 active:scale-95 group z-40"
                            :class="isUploading ? 'animate-pulse' : 'animate-bounce-subtle'">
                        
                        <!-- Ripple effect -->
                        <div class="absolute inset-0 rounded-full bg-white/20 scale-0 group-active:scale-100 transition-transform duration-300"></div>
                        
                        <!-- Icon with rotation -->
                        <div class="relative z-10 flex items-center justify-center">
                            <svg v-if="!isUploading" class="w-6 h-6 transition-transform duration-300 group-hover:rotate-12" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"/>
                            </svg>
                            <svg v-else class="w-6 h-6 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                            </svg>
                        </div>
                        
                        <!-- Tooltip -->
                        <div class="absolute bottom-full right-0 mb-2 px-3 py-1 bg-gray-900 text-white text-xs rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-200 whitespace-nowrap pointer-events-none">
                            {{ isUploading ? 'Uploading...' : 'Upload files' }}
                            <div class="absolute top-full right-4 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900"></div>
                        </div>
                    </button>
                </transition>

                </div>
            </div>

        </div>



    </div>

    <!-- Toast Notifications -->
    <transition
        enter-active-class="transition-all duration-300"
        enter-from-class="opacity-0 translate-y-4"
        enter-to-class="opacity-100 translate-y-0"
        leave-active-class="transition-all duration-200"
        leave-from-class="opacity-100 translate-y-0"
        leave-to-class="opacity-0 translate-y-4">
        <div v-if="toast.show" 
            class="fixed top-6 right-6 z-50 max-w-sm w-full">
            <div class="bg-white dark:bg-neutral-800 rounded-xl shadow-2xl border border-neutral-200 dark:border-neutral-700 p-4 flex items-start gap-3"
                :class="toast.type === 'success' ? 'border-green-200 dark:border-green-800' : 'border-red-200 dark:border-red-800'">
                
                <!-- Icon -->
                <div class="flex-shrink-0">
                    <div v-if="toast.type === 'success'" class="w-8 h-8 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center">
                        <svg class="w-5 h-5 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                        </svg>
                    </div>
                    <div v-else class="w-8 h-8 rounded-full bg-red-100 dark:bg-red-900/30 flex items-center justify-center">
                        <svg class="w-5 h-5 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </div>
                </div>
                
                <!-- Content -->
                <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-neutral-900 dark:text-neutral-100">{{ toast.title }}</p>
                    <p class="text-sm text-neutral-600 dark:text-neutral-400 mt-1">{{ toast.message }}</p>
                </div>
                
                <!-- Close button -->
                <button @click="hideToast" 
                    class="flex-shrink-0 w-6 h-6 rounded-lg flex items-center justify-center text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300 transition-colors duration-200">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                    </svg>
                </button>
            </div>
        </div>
    </transition>

</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, defineComponent } from 'vue';
import type { PropType } from 'vue';
import { useAuth } from '../../composables/useAuth';
import { useTheme } from '../../composables/useTheme';
import { getApiUrl } from '../../utils/apiUtils';
import axios from 'axios';
import IconXMark from '~icons/solar/close-circle-bold';
import Galaxy from '../../blocks/Backgrounds/Galaxy/Galaxy.vue';

// TreeNode Component Definition
const TreeNode = defineComponent({
    name: 'TreeNode',
    props: {
        node: {
            type: Object as PropType<any>,
            required: true
        },
        level: {
            type: Number,
            default: 0
        }
    },
    setup(props) {
        const toggleExpanded = () => {
            if (props.node.type === 'folder') {
                props.node.expanded = !props.node.expanded;
            }
        };

        const formatFileSize = (bytes: number): string => {
            if (bytes === 0) return "0 Bytes";
            const k = 1024;
            const sizes = ["Bytes", "KB", "MB", "GB"];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
        };

        return {
            toggleExpanded,
            formatFileSize
        };
    },
    template: `
        <div class="tree-node">
            <div 
                class="flex items-center gap-2 p-2 rounded-lg hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors cursor-pointer"
                :style="{ paddingLeft: \`\${level * 16 + 8}px\` }"
                @click="toggleExpanded"
            >
                <!-- Expand/Collapse Icon for folders -->
                <div v-if="node.type === 'folder'" class="w-4 h-4 flex items-center justify-center">
                    <svg 
                        class="w-3 h-3 transition-transform duration-200" 
                        :class="{ 'rotate-90': node.expanded }"
                        fill="none" 
                        stroke="currentColor" 
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                    </svg>
                </div>
                
                <!-- File/Folder Icon -->
                <div class="w-4 h-4 flex items-center justify-center">
                    <svg v-if="node.type === 'folder'" class="w-4 h-4 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
                    </svg>
                    <svg v-else class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                    </svg>
                </div>
                
                <!-- Node Name -->
                <span class="text-sm font-medium truncate flex-1">{{ node.name }}</span>
                
                <!-- Size -->
                <span class="text-xs text-neutral-500">{{ formatFileSize(node.size) }}</span>
            </div>
            
            <!-- Children -->
            <div v-if="node.type === 'folder' && node.expanded && node.children.length > 0">
                <TreeNode 
                    v-for="child in node.children" 
                    :key="child.path" 
                    :node="child" 
                    :level="level + 1" 
                />
            </div>
        </div>
    `
});

const { isAuthenticated } = useAuth();
const { isDark } = useTheme();

// Refs
const fileInput = ref<HTMLInputElement | null>(null);
const folderInput = ref<HTMLInputElement | null>(null);
const uploadSection = ref<HTMLElement | null>(null);
const { getSettings } = useAuth();

// Mouse tracking for interactive background
const mouseX = ref(0);
const mouseY = ref(0);

// Mouse move handler
const handleMouseMove = (e: MouseEvent) => {
    mouseX.value = e.clientX;
    mouseY.value = e.clientY;
};

// Toast notifications
const toast = ref({
    show: false,
    type: 'success' as 'success' | 'error',
    title: '',
    message: ''
});

// State
const selectedFiles = ref<File[]>([]);
const folderTree = ref<any>(null);
const isFolderMode = ref(false);
const isDragging = ref(false);
const isUploading = ref(false);
const progress = ref(0);
const uploadComplete = ref(false);

// New upload options
const transferMessage = ref('');
const recipientEmail = ref('');

// Upload statistics
const uploadSpeed = ref(0);
const timeRemaining = ref('Calculating...');
const estimatedTimeRemaining = ref(0);
const startTime = ref(0);
const lastLoaded = ref(0);
const lastTime = ref(0);
const uploadedUrl = ref("");
const uploadLink = ref("");
const copied = ref(false);
const message = ref({ text: "", type: "success" as "success" | "error" });
const maxUploadSize = ref<number>(104857600); // Default 100 MB

// Galaxy background loading state
const galaxyLoaded = ref(false);

const validityOptions = ref([
    { value: "7days", label: "7 Days", description: "One week" },
    { value: "1month", label: "1 Month", description: "30 days" },
    { value: "6months", label: "6 Months", description: "Half year" },
    { value: "1year", label: "1 Year", description: "12 months" },
    { value: "never", label: "Never", description: "Permanent" },
]);
const selectedValidity = ref("7days");
const maxAllowedValidity = ref("7days");

// Methods
const triggerFileInput = () => {
    fileInput.value?.click();
};

const triggerFolderInput = () => {
    folderInput.value?.click();
};

const buildFolderTree = (files: FileList) => {
    const tree: any = {
        name: 'Root',
        type: 'folder',
        children: [],
        path: '',
        size: 0,
        expanded: true
    };

    Array.from(files).forEach(file => {
        const path = (file as any).webkitRelativePath || file.name;
        const parts = path.split('/');
        let current = tree;

        parts.forEach((part: string, index: number) => {
            let child = current.children.find((c: any) => c.name === part);
            
            if (!child) {
                child = {
                    name: part,
                    type: index === parts.length - 1 ? 'file' : 'folder',
                    children: [],
                    path: parts.slice(0, index + 1).join('/'),
                    size: 0,
                    expanded: true,
                    file: index === parts.length - 1 ? file : null
                };
                current.children.push(child);
            }

            if (index === parts.length - 1) {
                child.size = file.size;
                current.size += file.size;
            } else {
                current = child;
            }
        });
    });

    // Calculate folder sizes recursively
    const calculateFolderSize = (node: any) => {
        if (node.type === 'file') return node.size;
        
        node.size = node.children.reduce((sum: number, child: any) => {
            return sum + calculateFolderSize(child);
        }, 0);
        
        return node.size;
    };
    
    calculateFolderSize(tree);
    
    return tree;
};

const onFileChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    if (target.files) {
        selectedFiles.value = Array.from(target.files);
        isFolderMode.value = false;
        folderTree.value = null;
    }
};

const onFolderChange = (event: Event) => {
    const target = event.target as HTMLInputElement;
    if (target.files) {
        selectedFiles.value = Array.from(target.files);
        isFolderMode.value = true;
        folderTree.value = buildFolderTree(target.files);
    }
};

const onDrop = (event: DragEvent) => {
    isDragging.value = false;
    if (event.dataTransfer?.files) {
        const files = Array.from(event.dataTransfer.files);
        selectedFiles.value = files;
        
        // Check if any file has webkitRelativePath (indicating folder structure)
        const hasFolderStructure = files.some(file => (file as any).webkitRelativePath && (file as any).webkitRelativePath.includes('/'));
        
        if (hasFolderStructure) {
            isFolderMode.value = true;
            folderTree.value = buildFolderTree(event.dataTransfer.files);
        } else {
            isFolderMode.value = false;
            folderTree.value = null;
        }
    }
};

const removeFile = (index: number) => {
    selectedFiles.value.splice(index, 1);
};

const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
};

const getFileExtension = (file: File): string => {
    return file.name.split(".").pop()?.toLowerCase() || "";
};

const getFileTypeColor = (file: File): string => {
    const ext = getFileExtension(file);
    const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'bmp', 'ico'];
    const videoExts = ['mp4', 'webm', 'ogg', 'avi', 'mov', 'mkv'];
    const audioExts = ['mp3', 'wav', 'ogg', 'm4a', 'flac', 'aac'];
    const docExts = ['pdf', 'doc', 'docx', 'txt', 'md', 'rtf'];
    const codeExts = ['js', 'ts', 'jsx', 'tsx', 'py', 'java', 'cpp', 'c', 'html', 'css', 'json', 'xml'];
    const archiveExts = ['zip', 'rar', '7z', 'tar', 'gz'];
    
    if (imageExts.includes(ext)) return 'bg-purple-500';
    if (videoExts.includes(ext)) return 'bg-red-500';
    if (audioExts.includes(ext)) return 'bg-green-500';
    if (docExts.includes(ext)) return 'bg-blue-500';
    if (codeExts.includes(ext)) return 'bg-yellow-500';
    if (archiveExts.includes(ext)) return 'bg-orange-500';
    return 'bg-gray-500';
};

const uploadFiles = async () => {
    if (selectedFiles.value.length === 0 || isUploading.value) return;

    isUploading.value = true;
    progress.value = 0;
    startTime.value = Date.now();
    lastLoaded.value = 0;
    lastTime.value = Date.now();
    uploadSpeed.value = 0;
    estimatedTimeRemaining.value = 0;
    timeRemaining.value = 'Calculating...';

    try {
        const formData = new FormData();
        selectedFiles.value.forEach((file) => {
            formData.append("files", file);
        });

        // Add upload options
        if (transferMessage.value) {
            formData.append("message", transferMessage.value);
        }
        if (recipientEmail.value) {
            formData.append("recipient_email", recipientEmail.value);
        }
        if (selectedValidity.value) {
            formData.append("validity", selectedValidity.value);
        }

        const response = await axios.post(
            "http://localhost:8080/api/upload",
            formData,
            {
                headers: {
                    "Content-Type": "multipart/form-data",
                    ...(isAuthenticated.value &&
                    localStorage.getItem("auth_token")
                        ? {
                              Authorization: `Bearer ${localStorage.getItem(
                                  "auth_token"
                              )}`,
                          }
                        : {}),
                },
                withCredentials: true,
                onUploadProgress: (progressEvent) => {
                    if (progressEvent.total) {
                        progress.value = Math.round(
                            (progressEvent.loaded * 100) / progressEvent.total
                        );

                        // Calculate upload speed and time remaining
                        const currentTime = Date.now();
                        const timeElapsed = (currentTime - lastTime.value) / 1000; // seconds
                        
                        if (timeElapsed > 0.5) { // Update every 0.5 seconds
                            const bytesUploaded = progressEvent.loaded - lastLoaded.value;
                            const speed = bytesUploaded / timeElapsed; // bytes per second
                            uploadSpeed.value = speed;

                            const remainingBytes = progressEvent.total - progressEvent.loaded;
                            const remainingSeconds = remainingBytes / speed;
                            estimatedTimeRemaining.value = remainingSeconds;
                            
                            if (remainingSeconds < 60) {
                                timeRemaining.value = `${Math.ceil(remainingSeconds)}s remaining`;
                            } else if (remainingSeconds < 3600) {
                                timeRemaining.value = `${Math.ceil(remainingSeconds / 60)}m remaining`;
                            } else {
                                timeRemaining.value = `${Math.ceil(remainingSeconds / 3600)}h remaining`;
                            }

                            lastLoaded.value = progressEvent.loaded;
                            lastTime.value = currentTime;
                        }
                    }
                },
            }
        );

        if (response.data.download_url) {
            progress.value = 100;
            uploadComplete.value = true;

            // Extract upload ID from download_url and create full URL
            const uploadId = response.data.download_url.split("/").pop();
            const fullUrl = getApiUrl(`download/${uploadId}`);
            uploadedUrl.value = fullUrl;
            uploadLink.value = fullUrl;

            message.value = {
                text: "Files uploaded successfully!",
                type: "success",
            };

            // Scroll to center the upload success section
            setTimeout(() => {
                uploadSection.value?.scrollIntoView({
                    behavior: "smooth",
                    block: "center",
                });
            }, 100);
        }
    } catch (error) {
        console.error("Upload error:", error);
        message.value = {
            text: "Upload failed. Please try again.",
            type: "error",
        };
        progress.value = 0;
    } finally {
        isUploading.value = false;

        // Clear message after 5 seconds instead of 3
        setTimeout(() => {
            message.value = { text: "", type: "success" };
            if (!isUploading.value) progress.value = 0;
        }, 5000);
    }
};

// Action buttons for upload success
const uploadNew = () => {
    uploadComplete.value = false;
    uploadedUrl.value = "";
    selectedFiles.value = [];
    progress.value = 0;
    message.value = { text: "", type: "success" };
};

const copyLink = async () => {
    try {
        await navigator.clipboard.writeText(uploadLink.value);
        copied.value = true;
        setTimeout(() => {
            copied.value = false;
        }, 2000);
    } catch (err) {
        message.value = { text: "Failed to copy link", type: "error" };
        setTimeout(() => {
            message.value = { text: "", type: "success" };
        }, 3000);
    }
};

const loadSettings = async () => {
    try {
        const settings = await getSettings();
        maxUploadSize.value = settings.maxUploadSize || 104857600;
        maxAllowedValidity.value = settings.maxValidity || "7days";

        // Filter validity options based on max allowed
        const validityOrder = ["7days", "1month", "6months", "1year", "never"];
        const maxIndex = validityOrder.indexOf(maxAllowedValidity.value);

        if (maxIndex !== -1) {
            const allowedOptions = validityOrder.slice(0, maxIndex + 1);
            validityOptions.value = validityOptions.value.filter((option) =>
                allowedOptions.includes(option.value)
            );
        }

        // Set default validity to the first available option
        if (validityOptions.value.length > 0) {
            selectedValidity.value = validityOptions.value[0].value;
        }
    } catch (error) {
        console.error("Error loading settings:", error);
    }
};

// Toast methods
const hideToast = () => {
    toast.value.show = false;
};

onMounted(() => {
    // Scroll to top smoothly when page loads
    loadSettings();
    window.scrollTo({ top: 0, behavior: "smooth" });

    // Add mouse tracking for interactive background
    window.addEventListener('mousemove', handleMouseMove);

    // Delay galaxy loading to prevent freeze on refresh
    setTimeout(() => {
        galaxyLoaded.value = true;
    }, 100);
});

onUnmounted(() => {
    // Clean up event listeners
    window.removeEventListener('mousemove', handleMouseMove);
});


</script>

<style scoped>
/* Custom Scrollbar */
.custom-scrollbar::-webkit-scrollbar {
    width: 6px;
}

.custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(156, 163, 175, 0.3);
    border-radius: 3px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(156, 163, 175, 0.5);
}

/* Smooth scrolling */
html {
    scroll-behavior: smooth;
}

/* Fade in animation */
@keyframes fade-in {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* Slide up animation */
@keyframes slide-up {
    from {
        opacity: 0;
        transform: translateY(30px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* Scale in animation */
@keyframes scale-in {
    from {
        opacity: 0;
        transform: scale(0.92);
    }
    to {
        opacity: 1;
        transform: scale(1);
    }
}

/* Bounce once animation */
@keyframes bounce-once {
    0%, 100% {
        transform: scale(1);
    }
    50% {
        transform: scale(1.1);
    }
}

/* Shimmer effect */
@keyframes shimmer {
    0% {
        background-position: -200% 0;
    }
    100% {
        background-position: calc(200px + 100%) 0;
    }
}

/* Spin animation */
@keyframes spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

/* Ping animation */
@keyframes ping {
    0% {
        transform: scale(1);
        opacity: 1;
    }
    75%, 100% {
        transform: scale(2);
        opacity: 0;
    }
}

/* Slow pulse animation */
@keyframes pulse-slow {
    0%, 100% {
        opacity: 0.3;
    }
    50% {
        opacity: 0.6;
    }
}

/* Animation classes */
.animate-fade-in {
    animation: fade-in 0.6s ease-out both;
}

.animate-slide-up {
    animation: slide-up 0.8s ease-out both;
}

.animate-scale-in {
    animation: scale-in 0.6s ease-out both;
}

.animate-bounce-once {
    animation: bounce-once 0.6s ease-out;
}

.animate-shimmer {
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
    background-size: 200% 100%;
    animation: shimmer 2s infinite;
}

.animate-spin {
    animation: spin 1s linear infinite;
}

.animate-ping {
    animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
}

.animate-pulse-slow {
    animation: pulse-slow 4s ease-in-out infinite;
}

/* Gradient radial background */
@keyframes gradient-radial {
    0%, 100% {
        background: radial-gradient(circle at 20% 50%, rgba(59, 130, 246, 0.1) 0%, transparent 50%),
                    radial-gradient(circle at 80% 20%, rgba(147, 51, 234, 0.1) 0%, transparent 50%),
                    radial-gradient(circle at 40% 80%, rgba(236, 72, 153, 0.1) 0%, transparent 50%);
    }
    50% {
        background: radial-gradient(circle at 80% 50%, rgba(59, 130, 246, 0.15) 0%, transparent 50%),
                    radial-gradient(circle at 20% 80%, rgba(147, 51, 234, 0.15) 0%, transparent 50%),
                    radial-gradient(circle at 60% 20%, rgba(236, 72, 153, 0.15) 0%, transparent 50%);
    }
}

/* Morphing animation for background elements */
@keyframes morph {
    0%, 100% {
        border-radius: 60% 40% 30% 70% / 60% 30% 70% 40%;
        transform: rotate(0deg) scale(1);
    }
    25% {
        border-radius: 30% 60% 70% 40% / 50% 60% 30% 60%;
        transform: rotate(90deg) scale(1.1);
    }
    50% {
        border-radius: 50% 40% 60% 50% / 40% 70% 50% 40%;
        transform: rotate(180deg) scale(0.9);
    }
    75% {
        border-radius: 40% 50% 40% 60% / 70% 40% 60% 50%;
        transform: rotate(270deg) scale(1.05);
    }
}

/* Morphing blob animations */
@keyframes morph-blob-1 {
    0%, 100% {
        border-radius: 60% 40% 30% 70% / 60% 30% 70% 40%;
        transform: scale(1) rotate(0deg);
    }
    25% {
        border-radius: 30% 60% 70% 40% / 50% 60% 30% 60%;
        transform: scale(1.1) rotate(90deg);
    }
    50% {
        border-radius: 50% 40% 60% 50% / 40% 70% 50% 40%;
        transform: scale(0.9) rotate(180deg);
    }
    75% {
        border-radius: 40% 50% 40% 60% / 70% 40% 60% 50%;
        transform: scale(1.05) rotate(270deg);
    }
}

@keyframes morph-blob-2 {
    0%, 100% {
        border-radius: 70% 30% 50% 50% / 35% 65% 35% 65%;
        transform: scale(1) rotate(0deg);
    }
    33% {
        border-radius: 40% 60% 60% 40% / 60% 40% 60% 40%;
        transform: scale(1.15) rotate(120deg);
    }
    66% {
        border-radius: 50% 50% 30% 70% / 50% 50% 70% 30%;
        transform: scale(0.85) rotate(240deg);
    }
}

@keyframes morph-blob-3 {
    0%, 100% {
        border-radius: 50% 50% 50% 50% / 50% 50% 50% 50%;
        transform: scale(1);
    }
    20% {
        border-radius: 60% 40% 40% 60% / 40% 60% 60% 40%;
        transform: scale(1.2);
    }
    40% {
        border-radius: 30% 70% 70% 30% / 70% 30% 30% 70%;
        transform: scale(0.8);
    }
    60% {
        border-radius: 45% 55% 55% 45% / 55% 45% 45% 55%;
        transform: scale(1.1);
    }
    80% {
        border-radius: 65% 35% 35% 65% / 35% 65% 65% 35%;
        transform: scale(0.9);
    }
}

@keyframes morph-blob-4 {
    0%, 100% {
        border-radius: 40% 60% 70% 30% / 60% 40% 30% 70%;
        transform: scale(1) rotate(0deg);
    }
    25% {
        border-radius: 70% 30% 40% 60% / 30% 70% 60% 40%;
        transform: scale(1.25) rotate(90deg);
    }
    50% {
        border-radius: 50% 50% 60% 40% / 40% 60% 50% 50%;
        transform: scale(0.75) rotate(180deg);
    }
    75% {
        border-radius: 60% 40% 50% 50% / 50% 50% 40% 60%;
        transform: scale(1.1) rotate(270deg);
    }
}

@keyframes morph-blob-5 {
    0%, 100% {
        border-radius: 50% 50% 50% 50% / 50% 50% 50% 50%;
        transform: scale(1);
    }
    30% {
        border-radius: 60% 40% 30% 70% / 40% 60% 70% 30%;
        transform: scale(1.3);
    }
    60% {
        border-radius: 30% 70% 60% 40% / 70% 30% 40% 60%;
        transform: scale(0.7);
    }
}

@keyframes morph-blob-6 {
    0%, 100% {
        border-radius: 45% 55% 65% 35% / 55% 45% 35% 65%;
        transform: scale(1) rotate(0deg);
    }
    20% {
        border-radius: 65% 35% 45% 55% / 35% 65% 55% 45%;
        transform: scale(1.2) rotate(72deg);
    }
    40% {
        border-radius: 35% 65% 55% 45% / 65% 35% 45% 55%;
        transform: scale(0.8) rotate(144deg);
    }
    60% {
        border-radius: 55% 45% 35% 65% / 45% 55% 65% 35%;
        transform: scale(1.15) rotate(216deg);
    }
    80% {
        border-radius: 50% 50% 60% 40% / 60% 40% 50% 50%;
        transform: scale(0.9) rotate(288deg);
    }
}

/* Text glow effect */
@keyframes text-glow {
    0%, 100% {
        text-shadow: 0 0 5px rgba(59, 130, 246, 0.3);
    }
    50% {
        text-shadow: 0 0 20px rgba(59, 130, 246, 0.6), 0 0 30px rgba(59, 130, 246, 0.4);
    }
}

/* Enhanced animations */
.animate-morph {
    animation: morph 20s ease-in-out infinite;
}

.animate-morph-blob-1 {
    animation: morph-blob-1 15s ease-in-out infinite;
}

.animate-morph-blob-2 {
    animation: morph-blob-2 18s ease-in-out infinite;
}

.animate-morph-blob-3 {
    animation: morph-blob-3 12s ease-in-out infinite;
}

.animate-morph-blob-4 {
    animation: morph-blob-4 20s ease-in-out infinite;
}

.animate-morph-blob-5 {
    animation: morph-blob-5 10s ease-in-out infinite;
}

.animate-morph-blob-6 {
    animation: morph-blob-6 16s ease-in-out infinite;
}

.animate-text-glow {
    animation: text-glow 3s ease-in-out infinite;
}

/* Glassmorphism effect */
.glass-effect {
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
}

/* Enhanced hover effects */
.hover-lift {
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.hover-lift:hover {
    transform: translateY(-2px) scale(1.02);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
}

/* Bounce subtle animation */
@keyframes bounce-subtle {
    0%, 100% {
        transform: translateY(0);
    }
    50% {
        transform: translateY(-4px);
    }
}

/* Parallax element animation */
@keyframes parallax-float {
    0%, 100% {
        transform: translateY(0px);
    }
    50% {
        transform: translateY(-10px);
    }
}

/* Background Animations */
@keyframes gradient-flow {
    0%, 100% {
        transform: translateX(0%) translateY(0%) scale(1);
        opacity: 0.15;
    }
    50% {
        transform: translateX(5%) translateY(-3%) scale(1.05);
        opacity: 0.25;
    }
}

@keyframes gradient-flow-reverse {
    0%, 100% {
        transform: translateX(0%) translateY(0%) scale(1);
        opacity: 0.15;
    }
    50% {
        transform: translateX(-5%) translateY(3%) scale(1.05);
        opacity: 0.25;
    }
}

@keyframes gradient-flow-slow {
    0%, 100% {
        transform: translateX(0%) translateY(0%) scale(1);
        opacity: 0.15;
    }
    50% {
        transform: translateX(3%) translateY(5%) scale(1.03);
        opacity: 0.2;
    }
}

@keyframes float-gentle {
    0%, 100% {
        transform: translateY(0px);
    }
    50% {
        transform: translateY(-10px);
    }
}

@keyframes float-delayed {
    0%, 100% {
        transform: translateY(0px) rotate(45deg);
    }
    50% {
        transform: translateY(-15px) rotate(47deg);
    }
}

@keyframes float-slow {
    0%, 100% {
        transform: translateY(0px);
    }
    50% {
        transform: translateY(-25px);
    }
}

@keyframes float-reverse {
    0%, 100% {
        transform: translateY(0px);
    }
    50% {
        transform: translateY(15px);
    }
}

@keyframes pattern-shift {
    0% {
        transform: translateX(0px) translateY(0px);
    }
    100% {
        transform: translateX(20px) translateY(-10px);
    }
}

/* Animation Classes */
.animate-gradient-flow {
    animation: gradient-flow 8s ease-in-out infinite;
}

.animate-gradient-flow-reverse {
    animation: gradient-flow-reverse 10s ease-in-out infinite;
}

.animate-gradient-flow-slow {
    animation: gradient-flow-slow 12s ease-in-out infinite;
}

.animate-float-gentle {
    animation: float-gentle 6s ease-in-out infinite;
}

.animate-float-delayed {
    animation: float-delayed 8s ease-in-out infinite 2s;
}

.animate-float-slow {
    animation: float-slow 10s ease-in-out infinite 1s;
}

.animate-float-reverse {
    animation: float-reverse 7s ease-in-out infinite 3s;
}

.animate-pattern-shift {
    animation: pattern-shift 15s linear infinite;
}

/* Loading skeleton animation */
@keyframes skeleton-loading {
    0% {
        background-position: -200px 0;
    }
    100% {
        background-position: calc(200px + 100%) 0;
    }
}

/* Dark mode skeleton */
.dark .animate-skeleton {
    background: linear-gradient(90deg, #374151 25%, #4b5563 50%, #374151 75%);
    background-size: 200px 100%;
}

/* Animated Background Styles */
@keyframes gradient-flow {
    0% {
        background-position: 0% 50%;
    }
    100% {
        background-position: 100% 50%;
    }
}

@keyframes gradient-flow-reverse {
    0% {
        background-position: 100% 50%;
    }
    100% {
        background-position: 0% 50%;
    }
}

@keyframes gradient-flow-slow {
    0% {
        background-position: 0% 50%;
    }
    100% {
        background-position: 100% 50%;
    }
}

@keyframes float-gentle {
    0% {
        transform: translateY(0px);
    }
    50% {
        transform: translateY(-10px);
    }
    100% {
        transform: translateY(0px);
    }
}

@keyframes float-delayed {
    0% {
        transform: translateY(0px);
    }
    50% {
        transform: translateY(-15px);
    }
    100% {
        transform: translateY(0px);
    }
}

@keyframes float-slow {
    0% {
        transform: translateY(0px);
    }
    50% {
        transform: translateY(-5px);
    }
    100% {
        transform: translateY(0px);
    }
}

@keyframes pattern-shift {
    0% {
        transform: translate(0) rotate(0);
    }
    100% {
        transform: translate(-10px, 10px) rotate(1deg);
    }
}


</style>