<script setup lang="ts">
import { ref } from 'vue'

interface SyncFile {
  name: string;
  path: string;
  size: string;
  status: 'synced' | 'pending' | 'out-of-sync';
  lastModified: string;
}

interface Project {
  id: string;
  name: string;
  filesCount: number;
  lastSynced: string;
  status: 'active' | 'archived';
  files: SyncFile[];
}

const projects = ref<Project[]>([
  {
    id: '1',
    name: 'cli-secrets-dashboard',
    filesCount: 3,
    lastSynced: '2 mins ago',
    status: 'active',
    files: [
      { name: '.env.local', path: 'web/.env.local', size: '1.2 KB', status: 'synced', lastModified: '2 mins ago' },
      { name: '.env.production', path: 'web/.env.production', size: '2.4 KB', status: 'synced', lastModified: '1 hour ago' },
      { name: 'ssh-config', path: '~/.ssh/config', size: '840 B', status: 'synced', lastModified: 'Yesterday' }
    ]
  },
  {
    id: '2',
    name: 'auth-service',
    filesCount: 2,
    lastSynced: '10 mins ago',
    status: 'active',
    files: [
      { name: '.env', path: 'crates/server/.env', size: '920 B', status: 'synced', lastModified: '10 mins ago' },
      { name: '.env.staging', path: 'crates/server/.env.staging', size: '1.1 KB', status: 'pending', lastModified: '5 mins ago' }
    ]
  },
  {
    id: '3',
    name: 'infra-k8s',
    filesCount: 1,
    lastSynced: '3 days ago',
    status: 'active',
    files: [
      { name: 'config', path: '~/.kube/config', size: '4.8 KB', status: 'out-of-sync', lastModified: '3 days ago' }
    ]
  }
])

const selectedProject = ref<Project>(projects.value[0])
const cliCommand = ref('env-vault push')
const isCopied = ref(false)

function copyCommand() {
  navigator.clipboard.writeText(cliCommand.value)
  isCopied.value = true
  setTimeout(() => isCopied.value = false, 2000)
}

function selectProject(project: Project) {
  selectedProject.value = project
}
</script>

<template>
  <div class="min-h-screen bg-slate-950 text-slate-100 font-sans selection:bg-indigo-500/30 selection:text-indigo-200">
    <!-- Header -->
    <header class="border-b border-slate-900 bg-slate-950/80 backdrop-blur-md sticky top-0 z-50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <div class="flex items-center space-x-3">
          <div class="h-9 w-9 rounded-xl bg-gradient-to-tr from-indigo-500 to-violet-600 flex items-center justify-center shadow-lg shadow-indigo-500/20">
            <svg class="h-5 w-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
            </svg>
          </div>
          <span class="text-xl font-bold tracking-tight bg-gradient-to-r from-white via-slate-200 to-slate-400 bg-clip-text text-transparent">
            cli-secrets
          </span>
        </div>
        <div class="flex items-center space-x-4">
          <div class="flex items-center space-x-1.5 px-3 py-1.5 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-xs font-semibold text-emerald-400">
            <span class="h-2 w-2 rounded-full bg-emerald-400 animate-pulse"></span>
            <span>Daemon Online</span>
          </div>
          <button class="px-4 py-1.5 text-sm font-semibold rounded-lg bg-indigo-600 hover:bg-indigo-500 active:bg-indigo-700 text-white transition-colors duration-200 shadow-lg shadow-indigo-600/25">
            Documentation
          </button>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 space-y-10">
      
      <!-- Hero Section -->
      <section class="text-center max-w-3xl mx-auto space-y-6">
        <h1 class="text-4xl sm:text-5xl font-extrabold tracking-tight">
          Zero-Knowledge Environment Sync
        </h1>
        <p class="text-lg text-slate-400 leading-relaxed">
          Secure, fast, and automated synchronization of your <code class="text-indigo-400 bg-indigo-950/40 border border-indigo-900/30 px-1.5 py-0.5 rounded text-sm font-mono">.env</code> configurations and SSH settings across development environments.
        </p>

        <!-- Command Bar -->
        <div class="flex items-center max-w-md mx-auto p-1.5 rounded-xl bg-slate-900/50 border border-slate-800 backdrop-blur-sm">
          <span class="text-slate-500 font-mono text-sm pl-3 select-none">$</span>
          <input 
            type="text" 
            readonly 
            :value="cliCommand" 
            class="w-full bg-transparent border-0 text-slate-200 font-mono text-sm px-2 focus:ring-0 focus:outline-none"
          />
          <button 
            @click="copyCommand"
            class="px-3 py-1.5 rounded-lg text-xs font-semibold bg-slate-800 hover:bg-slate-700 active:bg-slate-600 text-slate-300 transition-colors duration-150 flex items-center space-x-1"
          >
            <span>{{ isCopied ? 'Copied!' : 'Copy' }}</span>
          </button>
        </div>
      </section>

      <!-- Dashboard Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        
        <!-- Left Panel: Projects -->
        <div class="lg:col-span-1 space-y-4">
          <h2 class="text-lg font-bold text-slate-200 flex items-center space-x-2">
            <svg class="h-5 w-5 text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            <span>Active Projects</span>
          </h2>
          <div class="space-y-3">
            <div 
              v-for="project in projects" 
              :key="project.id"
              @click="selectProject(project)"
              :class="[
                'p-4 rounded-xl border transition-all duration-200 cursor-pointer flex flex-col justify-between space-y-3',
                selectedProject.id === project.id 
                  ? 'bg-slate-900 border-indigo-500/50 shadow-md shadow-indigo-950/20' 
                  : 'bg-slate-900/40 border-slate-850 hover:border-slate-800 hover:bg-slate-900/60'
              ]"
            >
              <div class="flex items-start justify-between">
                <div>
                  <h3 class="font-bold text-slate-100">{{ project.name }}</h3>
                  <p class="text-xs text-slate-500 font-mono mt-0.5">Synced {{ project.lastSynced }}</p>
                </div>
                <span class="text-xs px-2 py-0.5 rounded-md font-semibold bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
                  {{ project.filesCount }} files
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- Right Panel: Files List -->
        <div class="lg:col-span-2 space-y-4">
          <h2 class="text-lg font-bold text-slate-200 flex items-center space-x-2">
            <svg class="h-5 w-5 text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <span>Synchronized Files: {{ selectedProject.name }}</span>
          </h2>
          
          <div class="bg-slate-900/40 border border-slate-850 rounded-xl overflow-hidden backdrop-blur-sm">
            <table class="min-w-full divide-y divide-slate-850 text-left">
              <thead>
                <tr class="bg-slate-900/70 text-slate-400 text-xs font-semibold tracking-wider uppercase">
                  <th class="px-6 py-4">File Name</th>
                  <th class="px-6 py-4">Local Path</th>
                  <th class="px-6 py-4">Size</th>
                  <th class="px-6 py-4 text-right">Status</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-850/60 text-sm">
                <tr v-for="file in selectedProject.files" :key="file.path" class="hover:bg-slate-900/25 transition-colors duration-150">
                  <td class="px-6 py-4 font-mono font-bold text-slate-200">{{ file.name }}</td>
                  <td class="px-6 py-4 font-mono text-slate-450 text-xs">{{ file.path }}</td>
                  <td class="px-6 py-4 text-slate-400 text-xs">{{ file.size }}</td>
                  <td class="px-6 py-4 text-right">
                    <span 
                      :class="[
                        'inline-flex items-center space-x-1 px-2.5 py-0.5 rounded-full text-xs font-semibold border',
                        file.status === 'synced' ? 'bg-emerald-500/10 text-emerald-450 border-emerald-500/20' :
                        file.status === 'pending' ? 'bg-amber-500/10 text-amber-450 border-amber-500/20' :
                        'bg-rose-500/10 text-rose-450 border-rose-500/20'
                      ]"
                    >
                      <span :class="[
                        'h-1.5 w-1.5 rounded-full',
                        file.status === 'synced' ? 'bg-emerald-400' :
                        file.status === 'pending' ? 'bg-amber-400' :
                        'bg-rose-400'
                      ]"></span>
                      <span>{{ file.status }}</span>
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

      </div>
    </main>
  </div>
</template>
