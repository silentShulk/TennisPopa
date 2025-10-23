<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const registrationMessage = ref('');
const availabilityMessage = ref('');

// Calls the Rust backend command
async function runBackendCommand(commandName, commandArgs = {}) {
    try {
        const result = await invoke(commandName, {...commandArgs});
        return result;
    } catch (error) {
        throw error;
    } 
}

async function Aiiii() {
  await runBackendCommand('create_group');
}

async function handleRegistrationResponsesClick() {
    try {
        const formType = await runBackendCommand('get_registration_form_type');
        await runBackendCommand('main_get_forms_responses', { formType });
        registrationMessage.value = 'Operazione completata con successo';
        registrationMessageType.value = 'success';
    } catch (error) {
        registrationMessage.value = `Errore: ${error.message}`;
        registrationMessageType.value = 'error';
    }
}

async function handleAvailabilityResponsesClick() {
    try {
        const formType = await runBackendCommand('get_availability_form_type');
        await runBackendCommand('main_get_forms_responses', { formType });
        availabilityMessage.value = 'Operazione completata con successo';
        availabilityMessageType.value = 'success';
    } catch (error) {
        availabilityMessage.value = `Errore: ${error.message}`;
        availabilityMessageType.value = 'error';
    }
}

const registrationMessageType = ref('');
const availabilityMessageType = ref('');
</script>

<script>
export default {
  name: 'Page3'
}
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Ottieni Risposte Form</h1>
      <p>Recupera le risposte dai tuoi form compilati</p>
    </div>
    
    <div class="forms-container">
      <div class="form-card">
        <h2 class="card-title">Risposte Registrazione</h2>
        <div class="button-message-container">
          <button 
            class="generate-btn primary" 
            @click="handleRegistrationResponsesClick"
            :disabled="!!registrationMessage"
          >
            Ottieni risposte
          </button>
          <div v-if="registrationMessage" class="message-display" :class="registrationMessageType">
            {{ registrationMessage }}
          </div>
          <div v-else class="placeholder">Clicca per ottenere le risposte</div>
        </div>
      </div>
      <div>
        <button @click="Aiiii"> SKIBIDI RIZZ
        </button>
      </div>
      <div class="form-card">
        <h2 class="card-title">Risposte Disponibilità</h2>
        <div class="button-message-container">
          <button 
            class="generate-btn secondary" 
            @click="handleAvailabilityResponsesClick"
            :disabled="!!availabilityMessage"
          >
            Ottieni risposte
          </button>
          <div v-if="availabilityMessage" class="message-display" :class="availabilityMessageType">
            {{ availabilityMessage }}
          </div>
          <div v-else class="placeholder">Clicca per ottenere le risposte</div>
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

.button-message-container {
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

.message-display {
  padding: 0.75rem 1rem;
  border-radius: 8px;
  font-size: 0.95rem;
  text-align: center;
  width: 100%;
  max-width: 300px;
  word-break: break-word;
}

.message-display.success {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.message-display.error {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
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
}
</style>