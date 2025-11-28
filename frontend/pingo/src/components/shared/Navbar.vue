<template>
  <nav v-if="!isAuthPage" class="compact-navbar">
    <!-- add liquid glass effect -->
    <div class="navbar-content backdrop-blur-md bg-white/30 dark:bg-black/30 rounded-lg p-2">
      <!-- Compact Navigation Buttons -->
      <div class="nav-items">
        <template v-if="isAuthenticated">
          <router-link to="/" class="nav-item" title="Home">
            <IconHome class="icon" />
          </router-link>
          <router-link to="/account" class="nav-item" title="My Files">
            <IconFolder class="icon" />
          </router-link>
        </template>
      </div>

      <!-- Profile Section -->
      <div v-if="isAuthenticated" class="profile-section">
        <div class="profile-info">
          <span class="profile-name">{{ user?.username }}</span>
        </div>
        <router-link to="/account" class="nav-item" title="My Files">
        <div class="profile-avatar">
          <img 
            v-if="user?.avatar" 
            :src="getAssetUrl(user.avatar)" 
            :alt="user.username"
            class="avatar-img"
          />
          <span v-else class="avatar-initial">
            {{ (user?.username || 'U').charAt(0).toUpperCase() }}
          </span>
        </div>
        </router-link>

        <button @click="handleLogout" class="logout-icon" title="Sign Out">
          <IconLogout />
        </button>
      </div>

      <!-- Sign In Button (when not authenticated) -->
      <router-link v-else to="/auth" class="signin-btn">
        <IconLogin class="icon" />
        <span>Sign In</span>
      </router-link>
    </div>
  </nav>

  <!-- Theme Toggle Button (bottom-right) -->
  <button v-if="!isAuthPage" @click="toggleTheme" class="theme-toggle" :title="isDark ? 'Light Mode' : 'Dark Mode'">
    <IconLightMode v-if="isDark" class="icon" />
    <IconDarkMode v-else class="icon" />
  </button>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuth } from '../../composables/useAuth'
import { useTheme } from '../../composables/useTheme'
import { getAssetUrl } from '../../utils/apiUtils'
import IconHome from '~icons/mdi/home'
import IconFolder from '~icons/mdi/folder'
import IconShield from '~icons/mdi/shield-crown'
import IconLogout from '~icons/mdi/logout'
import IconLogin from '~icons/mdi/login'
import IconLightMode from '~icons/mdi/white-balance-sunny'
import IconDarkMode from '~icons/mdi/moon-waning-crescent'

const router = useRouter()
const route = useRoute()
const { user, isAuthenticated, logout } = useAuth()
const { isDark, toggleTheme } = useTheme()

const navbarTitle = ref('RootDrop')

const isAdmin = computed(() => user.value?.is_admin || false)
const isAuthPage = computed(() => route.path.startsWith('/auth'))

const handleLogout = async () => {
  await logout()
  router.push('/auth')
}

onMounted(async () => {
  try {
    const response = await fetch('/api/settings')
    if (response.ok) {
      const settings = await response.json()
      if (settings.navbar_title) {
        navbarTitle.value = settings.navbar_title
      }
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
})
</script>

<style scoped>
.compact-navbar {
  position: fixed;
  top: 1.5rem;
  right: 1.5rem;
  z-index: 1000;
}

.navbar-content {
  display: inline-flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 0.875rem;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
  border-radius: 50px;
  border: 1px solid rgba(0, 0, 0, 0.08);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.navbar-content:hover {
  box-shadow: 0 6px 30px rgba(0, 0, 0, 0.12);
}

:root[data-theme='dark'] .navbar-content {
  background: rgba(15, 23, 42, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
}

:root[data-theme='dark'] .navbar-content:hover {
  box-shadow: 0 6px 30px rgba(0, 0, 0, 0.7);
}

/* Navigation Items */
.nav-items {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.nav-item {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  color: #64748b;
  text-decoration: none;
  transition: all 0.2s ease;
  position: relative;
}

.nav-item::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.05);
  opacity: 0;
  transition: opacity 0.2s ease;
}

.nav-item:hover::before {
  opacity: 1;
}

.nav-item .icon {
  font-size: 1.25rem;
  position: relative;
  z-index: 1;
  transition: transform 0.2s ease;
}

.nav-item:hover .icon {
  transform: scale(1.1);
  color: #1f2937;
}

.nav-item.router-link-active {
  color: #6366f1;
}

.nav-item.router-link-active::before {
  background: rgba(99, 102, 241, 0.1);
  opacity: 1;
}

:root[data-theme='dark'] .nav-item {
  color: #94a3af;
}

:root[data-theme='dark'] .nav-item:hover .icon {
  color: #e5e7eb;
}

:root[data-theme='dark'] .nav-item::before {
  background: rgba(255, 255, 255, 0.08);
}

:root[data-theme='dark'] .nav-item.router-link-active {
  color: #818cf8;
}

:root[data-theme='dark'] .nav-item.router-link-active::before {
  background: rgba(129, 140, 248, 0.15);
}

/* Profile Section */
.profile-section {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  padding-left: 0.75rem;
  border-left: 1px solid rgba(0, 0, 0, 0.08);
}

:root[data-theme='dark'] .profile-section {
  border-left: 1px solid rgba(255, 255, 255, 0.1);
}

.profile-info {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
}

.profile-name {
  font-size: 0.875rem;
  font-weight: 600;
  color: #1f2937;
  white-space: nowrap;
}

:root[data-theme='dark'] .profile-name {
  color: #e5e7eb;
}

.profile-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
  color: white;
  font-weight: 600;
  font-size: 0.875rem;
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.3);
  transition: transform 0.2s ease;
  cursor: pointer;
}

.profile-avatar:hover {
  transform: scale(1.05);
}

.avatar-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.avatar-initial {
  user-select: none;
}

.logout-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: transparent;
  border: none;
  color: #dc2626;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
}

.logout-icon::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 50%;
  background: rgba(220, 38, 38, 0.08);
  opacity: 0;
  transition: opacity 0.2s ease;
}

.logout-icon:hover::before {
  opacity: 1;
}

.logout-icon .icon {
  font-size: 1.125rem;
  position: relative;
  z-index: 1;
  transition: transform 0.2s ease;
}

.logout-icon:hover .icon {
  transform: scale(1.1);
}

:root[data-theme='dark'] .logout-icon {
  color: #f87171;
}

:root[data-theme='dark'] .logout-icon::before {
  background: rgba(248, 113, 113, 0.15);
}

/* Sign In Button */
.signin-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: #1f2937;
  color: white;
  text-decoration: none;
  border-radius: 50px;
  font-size: 0.875rem;
  font-weight: 600;
  transition: all 0.2s ease;
}

.signin-btn:hover {
  background: #111827;
  transform: scale(1.02);
}

.signin-btn .icon {
  font-size: 1.125rem;
}

:root[data-theme='dark'] .signin-btn {
  background: rgba(99, 102, 241, 0.9);
}

:root[data-theme='dark'] .signin-btn:hover {
  background: #6366f1;
}

/* Responsive */
@media (max-width: 768px) {
  .compact-navbar {
    top: 1rem;
    right: 1rem;
  }

  .navbar-content {
    padding: 0.5rem 0.625rem;
    gap: 0.5rem;
  }

  .nav-item,
  .profile-avatar,
  .logout-icon {
    width: 32px;
    height: 32px;
  }

  .nav-item .icon {
    font-size: 1.125rem;
  }

  .profile-name {
    display: none;
  }

  .profile-section {
    padding-left: 0.5rem;
    gap: 0.5rem;
  }
}

@media (max-width: 480px) {
  .compact-navbar {
    top: 0.75rem;
    right: 0.75rem;
  }

  .nav-items {
    gap: 0.125rem;
  }
}

/* Theme Toggle Button */
.theme-toggle {
  position: fixed;
  bottom: 1.5rem;
  right: 1.5rem;
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  z-index: 999;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.theme-toggle:hover {
  background: rgba(255, 255, 255, 0.15);
  border-color: rgba(255, 255, 255, 0.3);
  transform: scale(1.05);
  box-shadow: 0 6px 30px rgba(0, 0, 0, 0.15);
}

.theme-toggle:active {
  transform: scale(0.95);
}

.theme-toggle .icon {
  font-size: 1.5rem;
  color: #64748b;
  transition: transform 0.3s ease;
}

.theme-toggle:hover .icon {
  transform: rotate(20deg);
}

:global(.dark) .theme-toggle {
  background: rgba(15, 23, 42, 0.8);
  border-color: rgba(71, 85, 105, 0.3);
}

:global(.dark) .theme-toggle .icon {
  color: #e2e8f0;
}

@media (max-width: 640px) {
  .theme-toggle {
    bottom: 1rem;
    right: 1rem;
    width: 48px;
    height: 48px;
  }

  .theme-toggle .icon {
    font-size: 1.25rem;
  }
}
</style>
