<template>
  <div class="container">
    <div class="group" @click="handleClick">
      <!-- Row 1 -->
      <p class="name">Nome 1</p>
      <div class="cell filled"></div>
      <div class="cell"></div>
      <div class="cell"></div>
      <div class="cell"></div>
      <!-- Row 2 -->
      <p class="name">Nome 2</p>
      <div class="cell"></div>
      <div class="cell filled"></div>
      <div class="cell"></div>
      <div class="cell"></div>
      <!-- Row 3 -->
      <p class="name">Nome 3</p>
      <div class="cell"></div>
      <div class="cell"></div>
      <div class="cell filled"></div>
      <div class="cell"></div>
      <!-- Row 4 -->
      <p class="name">Nome 4</p>
      <div class="cell"></div>
      <div class="cell"></div>
      <div class="cell"></div>
      <div class="cell filled"></div>
    </div>

    <input
      type="text"
      ref="textBox"
      :style="{ display: isVisible ? 'block' : 'none' }"
      placeholder="Inserire punteggio"
    >

  </div>
</template>




<script>
export default {
  name: 'Group',
  data() {
    return {
      isVisible: false // Reactive property to control input visibility
    };
  },
  methods: {
    handleClick(event) {
      if (event.target.classList.contains('cell')) {
        this.isVisible = true; // Toggle input visibility
      }
    }
  }
};
</script>

<style scoped>
.container {
  display: flex;
  justify-content: center; /* Centers horizontally */
  align-items: center; /* Centers vertically */
}

.group {
  display: grid;
  grid-template-columns: 100px repeat(4, 50px); /* Wider name column, 4 cell columns */
  grid-template-rows: repeat(4, 50px); /* 4 rows */
  gap: 0; /* No gap between grid cells */
  padding: 5px;
  margin: 20px auto; /* Center the grid */
  width: fit-content; /* Adjust width to content */
}

.cell {
  border: 2px solid black; /* Cell border */
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: white; /* Default cell background */
  cursor: pointer; /* Indicate clickability */
}

.name {
  background: transparent; /* Transparent background for names */
  border-top: 1px solid black;
  border-bottom: 1px solid black;
  display: flex;
  align-items: center; /* Vertically center text */
  justify-content: flex-start; /* Align text to left */
  padding-left: 10px; /* Add padding for readability */
  margin: 0; /* Remove default margins for grid alignment */
}

.filled {
  background-color: black; /* Filled cells are black */
}

input {

}
</style>