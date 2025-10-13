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

async function handleRegistrationResponsesClick() {
    const registrationFormType = await runBackendCommand('get_registration_form_type')

    runBackendCommand('main_get_forms_responses', { formType: registrationFormType})

    consoleLog('output', "Salvate risposte al form di registrazione nel database")
}
async function handleAvailabilityResponsesClick() {
    const availabilityFormType = await runBackendCommand('get_availability_form_type')

    runBackendCommand('main_get_forms_responses', { formType: availabilityFormType})

    consoleLog('output', "Salvate risposte al form di disponibilità nel database")
}

clearAndInitialize();

</script>

<script>
export default {
  name: 'Home'
}
</script>

<template>
  <div class="container">

    <h1>Home Page</h1>
    <p>Questa è la pagina principale</p>
    
    <div class="navigation-cards">
      <router-link to="/Giron" class="nav-card">
        <div class="card-content">
          <h3>Pagina 1</h3>
          <p>Vai alla prima pagina aggiuntiva</p>
        </div>
      </router-link>
      
      <router-link to="/Player" class="nav-card">
        <div class="card-content">
          <h3>Pagina 2</h3>
          <p>Vai alla seconda pagina aggiuntiva</p>
        </div>
      </router-link>
    </div>

    <div class="controls">
      <button @click="handleRegistrationClick()" class="success">
        Registrazione fasulla
      </button>
      <button @click="handleAvailabilityClick()" class="failure">
        Disponibilità
      </button>
      <button @click="clearAndInitialize" class="clear">
        Clear
      </button>
    </div>

    <div class="controls">
      <button @click="handleRegistrationResponsesClick()" class="success">
        Ottieni resposte registrazione
      </button>
      <button @click="handleAvailabilityResponsesClick()" class="failure">
        Ottieni risposte disponibilità
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

.home {
  text-align: center;
}

.navigation-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-top: 40px;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.nav-card {
  display: block;
  text-decoration: none;
  color: inherit;
}

.card-content {
  padding: 30px 20px;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  transition: all 0.3s ease;
  background: white;
}

.card-content:hover {
  border-color: #007acc;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
}

.card-content h3 {
  margin: 0 0 10px 0;
  color: #007acc;
}

.card-content p {
  margin: 0;
  color: #666;
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
  padding: 8px 12px;
  border: 1px solid #444;
  background-color: #333;
  color: #fff;
  cursor: pointer;
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