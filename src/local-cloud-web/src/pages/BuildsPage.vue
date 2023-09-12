<script setup>
import axios from "axios";
import {ref} from "vue";

const headers = [
  {
    label: 'name',
    field: 'name',
    name: 'name',
    align: 'left',
  },
  {
    label: 'dockerImageName',
    field: 'dockerImageName',
    name: 'dockerImageName',
    align: 'left',
  },
  {
    label: 'dockerContainerName',
    field: 'dockerContainerName',
    name: 'dockerContainerName',
    align: 'left',
  },
  {
    label: 'sourceHome',
    field: 'sourceHome',
    name: 'sourceHome',
    align: 'left',
  },
  {
    label: 'dockerFile',
    field: 'dockerFile',
    name: 'dockerFile',
    align: 'left',
  },
  {
    label: 'actions',
    field: 'actions',
    name: 'actions',
    align: 'left',
  },
]
const data = ref([])
axios.get('http://localhost:8088/cloud/projects')
  .then((response) => {
    data.value = response.data.projects
  })
  .catch((error) => {
    console.log(error)
  })
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
          <q-btn
            target="_blank"
            :href="'http://localhost:8088/cloud/projects/'+ props.row.name +'/republish'"
          >Rebuild & Publish</q-btn>
        </td>
      </template>
    </q-table>
  </div>
</template>

<style scoped>

</style>
