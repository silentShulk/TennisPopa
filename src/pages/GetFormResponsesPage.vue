<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const registrationMessage = ref('');
const availabilityMessage = ref('');
const showAvailabilityConfirm = ref(false);
const showRegistrationConfirm = ref(false);          // <-- NEW

const registrationMessageType = ref('');
const availabilityMessageType = ref('');

// Calls the Rust backend command
async function runBackendCommand(commandName, commandArgs = {}) {
  try {
    const result = await invoke(commandName, { ...commandArgs });
    return result;
  } catch (error) {
    throw error;
  }
}

/* ---------- REGISTRATION ---------- */
async function handleRegistrationResponsesClick() {
  showRegistrationConfirm.value = true;               // <-- open modal
}

async function handleRegistrationConfirm() {
  try {
    showRegistrationConfirm.value = false;
    const formType = await runBackendCommand('get_registration_form_type');
    await runBackendCommand('main_get_forms_responses', { formType });
    registrationMessage.value = 'Operazione completata con successo';
    registrationMessageType.value = 'success';
  } catch (error) {
    showRegistrationConfirm.value = false;
    registrationMessage.value = `Errore: ${error.message}`;
    registrationMessageType.value = 'error';
  }
}

function handleRegistrationCancel() {
  showRegistrationConfirm.value = false;
}

/* ---------- AVAILABILITY ---------- */
async function handleAvailabilityResponsesClick() {
  showAvailabilityConfirm.value = true;
}

async function handleAvailabilityConfirm() {
  try {
    showAvailabilityConfirm.value = false;
    const formType = await runBackendCommand('get_availability_form_type');
    await runBackendCommand('main_get_forms_responses', { formType });
    availabilityMessage.value = 'Operazione completata con successo';
    availabilityMessageType.value = 'success';
  } catch (error) {
    showAvailabilityConfirm.value = false;
    availabilityMessage.value = `Errore: ${error.message}`;
    availabilityMessageType.value = 'error';
  }
}

function handleAvailabilityCancel() {
  showAvailabilityConfirm.value = false;
}
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
      <!-- REGISTRATION CARD -->
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

      <!-- AVAILABILITY CARD -->
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

    <!-- REGISTRATION CONFIRMATION MODAL -->
    <div v-if="showRegistrationConfirm" class="modal-backdrop" @click="handleRegistrationCancel">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>ATTENZIONE!!!</h3>
        </div>
        <div class="modal-body">
          <p>Se procedi i gironi saranno resettati e ricreati</p>
          <p>Vuoi procedere?</p>
        </div>
        <div class="modal-footer">
          <button class="confirm-btn" @click="handleRegistrationConfirm">
            Ottieni risposte e ricrea gironi
          </button>
          <button class="cancel-btn" @click="handleRegistrationCancel">
            Annulla
          </button>
        </div>
      </div>
    </div>

    <!-- AVAILABILITY CONFIRMATION MODAL (unchanged) -->
    <div v-if="showAvailabilityConfirm" class="modal-backdrop" @click="handleAvailabilityCancel">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>ATTENZIONE!!!</h3>
        </div>
        <div class="modal-body">
          <p>Se procedi tutte le risposte ottenute precedentemente saranno sovrascritte con le nuove</p>
          <p>Vuoi continuare?</p>
        </div>
        <div class="modal-footer">
          <button class="confirm-btn" @click="handleAvailabilityConfirm">
            Sovrascrivi con nuove risposte
          </button>
          <button class="cancel-btn" @click="handleAvailabilityCancel">
            Annulla
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* ---------- (your original styles – unchanged) ---------- */
.page-container { min-height: 100vh; background: linear-gradient(135deg, #4facfe 0%, #00acb5 100%); font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; }
.page-header { background: rgba(255,255,255,0.95); padding:2rem; text-align:center; box-shadow:0 4px 20px rgba(0,0,0,.1); backdrop-filter:blur(10px); }
.page-header h1 { color:#2c3e50; margin:0 0 .5rem; font-size:2.8rem; font-weight:300; letter-spacing:-.5px; }
.page-header p { color:#7f8c8d; font-size:1.1rem; margin:0; }

.forms-container { display:grid; grid-template-columns:repeat(auto-fit,minmax(400px,1fr)); gap:2rem; padding:3rem 2rem; max-width:1200px; margin:0 auto; }
.form-card { background:rgba(255,255,255,.95); border-radius:16px; padding:2rem; box-shadow:0 8px 32px rgba(0,0,0,.1); backdrop-filter:blur(10px); border:1px solid rgba(255,255,255,.2); transition:transform .3s,box-shadow .3s; }
.form-card:hover { transform:translateY(-4px); box-shadow:0 12px 40px rgba(0,0,0,.15); }

.card-title { color:#2c3e50; margin:0 0 1.5rem; font-size:1.6rem; font-weight:400; text-align:center; border-bottom:2px solid #e9ecef; padding-bottom:.5rem; }
.button-message-container { display:flex; flex-direction:column; align-items:center; gap:1rem; }

.generate-btn { padding:12px 24px; border:none; border-radius:12px; font-size:1rem; font-weight:600; cursor:pointer; transition:all .3s; min-width:140px; text-transform:uppercase; letter-spacing:.5px; }
.generate-btn.primary { background:linear-gradient(135deg,#4facfe 0%,#00f2fe 100%); color:#fff; box-shadow:0 4px 15px rgba(79,172,254,.4); }
.generate-btn.primary:hover:not(:disabled) { transform:translateY(-2px); box-shadow:0 6px 20px rgba(79,172,254,.5); }
.generate-btn.secondary { background:linear-gradient(135deg,#43e97b 0%,#38f9d7 100%); color:#fff; box-shadow:0 4px 15px rgba(67,233,123,.4); }
.generate-btn.secondary:hover:not(:disabled) { transform:translateY(-2px); box-shadow:0 6px 20px rgba(67,233,123,.5); }
.generate-btn:disabled { opacity:.6; cursor:not-allowed; transform:none; }

.message-display { padding:.75rem 1rem; border-radius:8px; font-size:.95rem; text-align:center; width:100%; max-width:300px; word-break:break-word; }
.message-display.success { background:#d4edda; color:#155724; border:1px solid #c3e6cb; }
.message-display.error { background:#f8d7da; color:#721c24; border:1px solid #f5c6cb; }
.placeholder { color:#adb5bd; font-style:italic; text-align:center; width:100%; padding:1rem; }

/* Modal */
.modal-backdrop { position:fixed; top:0; left:0; width:100%; height:100%; background:rgba(0,0,0,.5); display:flex; justify-content:center; align-items:center; z-index:1000; }
.modal-content { background:#fff; border-radius:12px; padding:2rem; max-width:500px; width:90%; box-shadow:0 10px 40px rgba(0,0,0,.2); text-align:center; }
.modal-header h3 { color:#e74c3c; margin:0 0 1rem; font-size:1.5rem; }
.modal-body p { color:#2c3e50; margin:.5rem 0; line-height:1.5; }
.modal-footer { display:flex; gap:1rem; justify-content:center; margin-top:1.5rem; }
.confirm-btn, .cancel-btn { padding:10px 20px; border:none; border-radius:8px; font-size:1rem; font-weight:600; cursor:pointer; transition:all .3s; min-width:200px; }
.confirm-btn { background:linear-gradient(135deg,#27ae60 0%,#2ecc71 100%); color:#fff; }
.confirm-btn:hover { transform:translateY(-2px); box-shadow:0 4px 15px rgba(46,204,113,.4); }
.cancel-btn { background:linear-gradient(135deg,#e74c3c 0%,#c0392b 100%); color:#fff; }
.cancel-btn:hover { transform:translateY(-2px); box-shadow:0 4px 15px rgba(231,76,60,.4); }

/* Responsive */
@media (max-width:768px) {
  .forms-container { grid-template-columns:1fr; padding:2rem 1rem; gap:1.5rem; }
  .form-card { padding:1.5rem; }
  .card-title { font-size:1.4rem; }
  .generate-btn { min-width:120px; }
  .modal-content { padding:1.5rem; margin:1rem; }
  .modal-footer { flex-direction:column; }
  .confirm-btn, .cancel-btn { min-width:auto; }
}
</style>