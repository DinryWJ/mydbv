import { defineStore } from "pinia";

export const useActivateDatabaseStore = defineStore('activateDatabase', {
    state: () => {
        return {
            activeDatabase: []
        }
    },

    actions: {
        addActiveDatabase(database) {
            console.info('addActiveDatabase', database);
            this.activeDatabase.push(database);
        },

        removeActiveDatabase(id) {
            this.activeDatabase = this.activeDatabase.filter(db => db.id !== id);
        },

        getAll() {
            console.log('getAll', this.activeDatabase);
            return this.activeDatabase;
        }
    }
})