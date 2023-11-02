import { grpc } from "@improbable-eng/grpc-web";
import { SingleUserRetrieve, EmptyReqRes, GrpcWebImpl, AllUserRetrieve, StaffUsersClientImpl } from "./generated/proto/staffusers";


const HOST = "http://localhost:3000";

export class GetAllUsers {
    private rpc: GrpcWebImpl;
    private client: StaffUsersClientImpl;

    constructor(usertoken: string) {
        const metadata = new grpc.Metadata();
        metadata.append('authentication', `Bearer ${usertoken}`);
        this.rpc = new GrpcWebImpl(HOST, { metadata });
        this.client = new StaffUsersClientImpl(this.rpc);
    }

    async fetchAllUsers(usertoken: string) {
        try {
        
            const headers = new grpc.Metadata();
            headers.append('authentication', `Bearer ${usertoken}`);
            const response = await this.client.GetAllUsers(headers);
            
            // Access the list of users from the response (placeholder code).
            const usersList = response.users;
            
            // Process the list of users as needed.
            return usersList;
        } catch (error) {
            // Handle any errors that may occur during the gRPC call.
            console.error('Error fetching users:', error);
            throw error;
        }
    }
}
