<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const consoleOutput = ref([]);

// Clears console and adds an initial message
const clearAndInitialize = () => {
    consoleOutput.value = [];
};

function consoleLog(resultType, functionResult) {
  const formatted =
    typeof functionResult === 'object'
      ? JSON.stringify(functionResult, null, 2)
      : functionResult;

  consoleOutput.value.push({
    type: resultType,
    message: `Result: ${formatted}`,
  });
}

// Calls the Rust backend command
async function runBackendCommand(commandName, commandArgs = {}) {
    try {
        const result = await invoke(commandName, {...commandArgs});

        return result
    } catch (error) {
        consoleLog('error', error)

        throw error
    } 
}

async function handleRegistrationClick() {
    // get the registration form first
    // const registrationForm = await runBackendCommand('get_registration_form');

    // // call create_form with the form as argument
    // const registrationUrl = await runBackendCommand('create_form', { formInfo: registrationForm });

    // consoleLog('output', registrationUrl)

    consoleLog('output', "skibidi piangi no registrazione per te, no sigma")
}
async function handleAvailabilityClick() {
    // get the registration form first
    const avaiabilityForm = await runBackendCommand('get_availability_form');
    
    // call create_form with the form as argument
    const avaiabilityUrl = await runBackendCommand('create_form', { formInfo: avaiabilityForm });

    consoleLog('output', avaiabilityUrl)

}

clearAndInitialize();

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
    </div>
    
    <div class="controls">
      <button class="success" @click="handleRegistrationClick()">
        Registrazione fasulla
      </button>
      <button class="failure" @click="handleAvailabilityClick()">
        Disponibilità
      </button>
      <button class="clear" @click="clearAndInitialize">
        Clear
      </button>
    </div>

    <div class="console">
      <div v-for="(item, index) in consoleOutput" :key="index" :class="['log-entry', item.type]">
        {{ item.message }}
      </div>
    </div>

  </div>
</template>

<style scoped>

.page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #4facfe 0%, #00acb5 100%);
}

.page-header {
  background: rgba(255, 255, 255, 0.95);
  padding: 20px;
  text-align: center;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.page-header h1 {
  color: #2c3e50;
  margin-bottom: 10px;
  font-size: 2.5rem;
}

.page-header p {
  color: #7f8c8d;
  font-size: 1.2rem;
}

.page-content {
  padding: 40px;
}

.content-card {
  background: white;
  border-radius: 10px;
  padding: 30px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
  max-width: 1000px;
  margin: 0 auto;
}

.content-card h2 {
  color: #2c3e50;
  margin-bottom: 20px;
  font-size: 1.8rem;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-top: 30px;
}

.feature-card {
  background: #f8f9fa;
  padding: 20px;
  border-radius: 8px;
  border-left: 4px solid #3498db;
}

.feature-card h3 {
  color: #2c3e50;
  margin-bottom: 10px;
}

/* Minimalist, focused styling */
.container {
  padding: 10px;
  font-family: monospace;
  max-width: 600px;
  margin: 0 auto;
}

.controls {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
}

.controls button {
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  font-family: inherit;
}

/* Console Styling */
.console {
  height: 200px;
  padding: 5px;
  background-color: #1e1e1e;
  border: 1px solid #555;
  overflow-y: auto;
  font-size: 0.85em;
}

/* Log Entry Styling */
.log-entry.output {
  color: #00ff00; /* Green */
}

.log-entry.error {
  color: #ff5555; /* Red */
}

.log-entry.info {
  color: #8888ff; /* Blue */
}
</style>