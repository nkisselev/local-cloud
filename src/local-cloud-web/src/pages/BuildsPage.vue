<script setup>
const headers = [
  {
    label: 'name',
    field: 'name',
    name: 'name',
  },
  {
    label: 'actions',
    field: 'actions',
    name: 'actions',
  },
]
const data = [
  {
    name: 'project 1',
  },
  {
    name: 'project 2',
  },
]
</script>

<template>
  <div>
    <h1>Projects</h1>
    <q-table
      :columns="headers"
      :rows="data"
      row-key="name"
    >
      <template v-slot:body-cell-actions="props">
        <td :props="props">
          <q-btn>Rebuild & Publish</q-btn>
        </td>
      </template>
    </q-table>
  </div>
</template>

<style scoped>

</style>
