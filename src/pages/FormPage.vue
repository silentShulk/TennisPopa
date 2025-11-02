<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const registrationUrl = ref('');
const availabilityUrl = ref('');

// Stato per il modal
const showModal = ref(false);
const modalStep = ref(1); // 1 = prima conferma, 2 = seconda
const pendingAction = ref(null); // 'registration' | 'availability'

// Chiamata al backend Rust
async function runBackendCommand(commandName, commandArgs = {}) {
  try {
    const result = await invoke(commandName, { ...commandArgs });
    return result;
  } catch (error) {
    throw error;
  }
}

// Gestione click Registrazione
async function handleRegistrationClick() {
  try {
    const registrationForm = await runBackendCommand('get_registration_form');
    const url = await runBackendCommand('create_form', { formInfo: registrationForm });
    registrationUrl.value = url;
  } catch (error) {
    registrationUrl.value = `Error: ${error.message}`;
  }
}

// Gestione click Disponibilità
async function handleAvailabilityClick() {
  try {
    const availabilityForm = await runBackendCommand('get_availability_form');
    const url = await runBackendCommand('create_form', { formInfo: availabilityForm });
    availabilityUrl.value = url;
  } catch (error) {
    availabilityUrl.value = `Error: ${error.message}`;
  }
}

// Copia negli appunti
async function copyToClipboard(text) {
  try {
    await navigator.clipboard.writeText(text);
  } catch (err) {
    console.error('Failed to copy: ', err);
  }
}

// --- MODAL LOGIC ---
function openConfirm(action) {
  pendingAction.value = action;
  modalStep.value = 1;
  showModal.value = true;
}

function proceedToSecond() {
  modalStep.value = 2;
}

async function executeAction() {
  showModal.value = false;
  if (pendingAction.value === 'registration') {
    await handleRegistrationClick();
  } else if (pendingAction.value === 'availability') {
    await handleAvailabilityClick();
  }
  pendingAction.value = null;
  modalStep.value = 1;
}

function cancelAction() {
  showModal.value = false;
  pendingAction.value = null;
  modalStep.value = 1;
}
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Creazione Form</h1>
      <p>Genera link personalizzati per i tuoi form in un clic</p>
    </div>

    <div class="forms-container">
      <!-- CARD REGISTRAZIONE -->
      <div class="form-card">
        <h2 class="card-title">Registrazione</h2>
        <div class="button-link-container">
          <button
            class="generate-btn primary"
            @click="openConfirm('registration')"
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

      <!-- CARD DISPONIBILITÀ -->
      <div class="form-card">
        <h2 class="card-title">Disponibilità</h2>
        <div class="button-link-container">
          <button
            class="generate-btn secondary"
            @click="openConfirm('availability')"
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

    <!-- MODAL DI CONFERMA -->
    <teleport to="body">
      <div v-if="showModal" class="modal-overlay" @click="cancelAction">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3 v-if="modalStep === 1" class="warning-title">
              ATTENZIONE !!!
            </h3>
            <h3 v-else class="danger-title">
              ELIMINARE TUTTO???
            </h3>
          </div>

          <div class="modal-body">
            <p v-if="modalStep === 1" class="warning-text">
              <strong>ATTENZIONE !!!</strong><br>
              <strong>ATTENZIONE !!!</strong><br>
              Se clicchi questo tasto tutti i dati registrati per <strong>TUTTI I GIOCATORI</strong> saranno eliminati.
            </p>
            <p v-else class="danger-text">
              <strong>(Seconda Conferma)</strong>
            </p>
          </div>

          <div class="modal-actions">
            <button
              v-if="modalStep === 1"
              @click="proceedToSecond"
              class="btn-proceed"
            >
              Procedi
            </button>
            <button
              v-else
              @click="executeAction"
              class="btn-confirm"
            >
              CONFERMA
            </button>
            <button @click="cancelAction" class="btn-cancel">
              Annulla
            </button>
          </div>
        </div>
      </div>
    </teleport>
  </div>
</template>

<style scoped>
/* === LAYOUT PRINCIPALE === */
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

/* === MODAL STYLES === */
.modal-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.3s ease;
}

.modal-content {
  background: rgba(255, 255, 255, 0.98);
  border-radius: 16px;
  padding: 1.8rem;
  width: 90%;
  max-width: 420px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.3);
  animation: slideUp 0.4s ease;
  text-align: center;
}

.modal-header h3 {
  margin: 0 0 1rem;
  font-weight: 700;
  font-size: 1.6rem;
  letter-spacing: 1px;
}

.warning-title {
  color: #e67e22;
  text-shadow: 0 0 10px rgba(230, 126, 34, 0.3);
}

.danger-title {
  color: #e74c3c;
  text-shadow: 0 0 10px rgba(231, 76, 60, 0.3);
}

.modal-body p {
  margin: 0 0 1.5rem;
  line-height: 1.6;
  font-size: 1rem;
}

.warning-text {
  color: #d35400;
}

.danger-text {
  color: #c0392b;
  font-weight: 600;
}

.modal-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.modal-actions button {
  padding: 10px 20px;
  border: none;
  border-radius: 10px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 100px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.btn-proceed {
  background: linear-gradient(135deg, #43e97b, #38f9d7);
  color: white;
  box-shadow: 0 4px 15px rgba(67, 233, 123, 0.4);
}

.btn-proceed:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(67, 233, 123, 0.5);
}

.btn-confirm {
  background: linear-gradient(135deg, #e74c3c, #c0392b);
  color: white;
  box-shadow: 0 4px 15px rgba(231, 76, 60, 0.4);
}

.btn-confirm:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(231, 76, 60, 0.5);
}

.btn-cancel {
  background: #6c757d;
  color: white;
}

.btn-cancel:hover {
  background: #5a6268;
  transform: translateY(-2px);
}

/* Animazioni */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from { transform: translateY(50px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

/* Responsive */
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
  .modal-content {
    padding: 1.5rem;
  }
}
</style>