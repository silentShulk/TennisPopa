<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const registrationUrl = ref('');
const availabilityUrl = ref('');

function SKIBIDI(){
  invoke ('create_groups', {});
}

// Calls the Rust backend command
async function runBackendCommand(commandName, commandArgs = {}) {
    try {
        const result = await invoke(commandName, {...commandArgs});
        return result;
    } catch (error) {
        throw error;
    } 
}

async function handleRegistrationClick() {
    try {
        const registrationForm = await runBackendCommand('get_registration_form');
        const url = await runBackendCommand('create_form', { formInfo: registrationForm });
        registrationUrl.value = url;
    } catch (error) {
        registrationUrl.value = `Error: ${error.message}`;
    }
}

async function handleAvailabilityClick() {
    try {
        const availabilityForm = await runBackendCommand('get_availability_form');
        const url = await runBackendCommand('create_form', { formInfo: availabilityForm });
        availabilityUrl.value = url;
    } catch (error) {
        availabilityUrl.value = `Error: ${error.message}`;
    }
}

async function copyToClipboard(text) {
    try {
        await navigator.clipboard.writeText(text);
        // Optional: Add a visual feedback here if needed
    } catch (err) {
        console.error('Failed to copy: ', err);
    }
}
</script>

<script>
export default {
  name: 'Home'
}
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Creazione Form</h1>
      <p>Genera link personalizzati per i tuoi form in un clic</p>
    </div>
    <button @click="SKIBIDI" >aaaaaaa</button>
    <div class="forms-container">
      <div class="form-card">
        <h2 class="card-title">Registrazione</h2>
        <div class="button-link-container">
          <button 
            class="generate-btn primary" 
            @click="handleRegistrationClick"
            :disabled="!!registrationUrl"
          >
            Genera Link
          </button>
          <div v-if="registrationUrl" class="link-display">
            <a :href="registrationUrl" target="_blank" rel="noopener noreferrer" class="url-link">
              {{ registrationUrl }}
            </a>
            <button class="copy-btn" @click="copyToClipboard(registrationUrl)">
              📋 Copia
            </button>
          </div>
          <div v-else class="placeholder">Clicca per generare il link</div>
        </div>
      </div>

      <div class="form-card">
        <h2 class="card-title">Disponibilità</h2>
        <div class="button-link-container">
          <button 
            class="generate-btn secondary" 
            @click="handleAvailabilityClick"
            :disabled="!!availabilityUrl"
          >
            Genera Link
          </button>
          <div v-if="availabilityUrl" class="link-display">
            <a :href="availabilityUrl" target="_blank" rel="noopener noreferrer" class="url-link">
              {{ availabilityUrl }}
            </a>
            <button class="copy-btn" @click="copyToClipboard(availabilityUrl)">
              📋 Copia
            </button>
          </div>
          <div v-else class="placeholder">Clicca per generare il link</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #4facfe 0%, #00acb5 100%);
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.page-header {
  background: rgba(255, 255, 255, 0.95);
  padding: 2rem;
  text-align: center;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
}

.page-header h1 {
  color: #2c3e50;
  margin: 0 0 0.5rem 0;
  font-size: 2.8rem;
  font-weight: 300;
  letter-spacing: -0.5px;
}

.page-header p {
  color: #7f8c8d;
  font-size: 1.1rem;
  margin: 0;
}

.forms-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: 2rem;
  padding: 3rem 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.form-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  padding: 2rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.form-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
}

.card-title {
  color: #2c3e50;
  margin: 0 0 1.5rem 0;
  font-size: 1.6rem;
  font-weight: 400;
  text-align: center;
  border-bottom: 2px solid #e9ecef;
  padding-bottom: 0.5rem;
}

.button-link-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.generate-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 12px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 140px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.generate-btn.primary {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(79, 172, 254, 0.4);
}

.generate-btn.primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(79, 172, 254, 0.5);
}

.generate-btn.secondary {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(67, 233, 123, 0.4);
}

.generate-btn.secondary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(67, 233, 123, 0.5);
}

.generate-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

.link-display {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  align-items: center;
  justify-content: center;
  width: 100%;
}

.url-link {
  color: #4facfe;
  text-decoration: none;
  font-family: monospace;
  font-size: 0.85rem;
  word-break: break-all;
  max-width: 250px;
  text-align: center;
  padding: 0.5rem;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #e9ecef;
  transition: color 0.3s ease;
}

.url-link:hover {
  color: #00f2fe;
  text-decoration: underline;
}

.copy-btn {
  padding: 8px 12px;
  border: none;
  border-radius: 8px;
  background: #6c757d;
  color: white;
  cursor: pointer;
  font-size: 0.9rem;
  transition: background 0.3s ease;
  white-space: nowrap;
}

.copy-btn:hover {
  background: #5a6268;
}

.placeholder {
  color: #adb5bd;
  font-style: italic;
  text-align: center;
  width: 100%;
  padding: 1rem;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .forms-container {
    grid-template-columns: 1fr;
    padding: 2rem 1rem;
    gap: 1.5rem;
  }

  .form-card {
    padding: 1.5rem;
  }

  .card-title {
    font-size: 1.4rem;
  }

  .generate-btn {
    min-width: 120px;
  }

  .url-link {
    max-width: 100%;
    font-size: 0.8rem;
  }
}
</style>