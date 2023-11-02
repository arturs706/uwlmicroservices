import { Component, createSignal, onMount } from "solid-js";
import styles from "../../css/Herosection.module.css"
import { GetAllUsers } from "../../src/api"; // replace with actual path to api.tsx
import { SingleUserRetrieve } from "../../src/generated/proto/staffusers"; // replace with actual path to proto file

const Herosection: Component = () => {
    const [users, setUsers] = createSignal<SingleUserRetrieve[]>([]);
    onMount(async () => {
        console.log('Fetching users...');
        const usertoken = '1'; // replace with actual user token
        const getAllUsersInstance = new GetAllUsers(usertoken);
        console.log('getAllUsersInstance:', getAllUsersInstance);
        try {
            const usersList = await getAllUsersInstance.fetchAllUsers(usertoken);
            console.log('usersList:', usersList);
            setUsers(usersList);
        } catch (error) {
            console.error('Error fetching users:', error);
        }
    });

    return (
        <main>
            {users().map(user => (
                <div>{user.name}</div> // replace with actual user property
            ))}
        </main>
    )
}

export default Herosection;
