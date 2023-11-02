/* eslint-disable */
import { grpc } from "@improbable-eng/grpc-web";
import { BrowserHeaders } from "browser-headers";
import _m0 from "protobufjs/minimal";

export const protobufPackage = "staffusers";

export interface RegisterUserRequest {
  name: string;
  username: string;
  mobPhone: string;
  passwd: string;
}

export interface RegisterUserResponse {
  name: string;
  message: string;
}

export interface SingleUserRetrieve {
  userId: string;
  name: string;
  username: string;
  mobPhone: string;
  accessLevel: string;
  status: string;
  aCreated: string;
}

export interface AllUserRetrieve {
  users: SingleUserRetrieve[];
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface GetUserRequest {
  userId: number;
}

export interface DeleteUserRequest {
  userId: number;
}

export interface ChangePasswordRequest {
  userId: number;
  oldPassword: string;
  newPassword: string;
}

export interface AssignRoleRequest {
  userId: number;
  newRole: string;
}

export interface SuspendAccountRequest {
  userId: number;
}

export interface EmptyReqRes {
}

function createBaseRegisterUserRequest(): RegisterUserRequest {
  return { name: "", username: "", mobPhone: "", passwd: "" };
}

export const RegisterUserRequest = {
  encode(message: RegisterUserRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.username !== "") {
      writer.uint32(18).string(message.username);
    }
    if (message.mobPhone !== "") {
      writer.uint32(26).string(message.mobPhone);
    }
    if (message.passwd !== "") {
      writer.uint32(34).string(message.passwd);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RegisterUserRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRegisterUserRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.username = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.mobPhone = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.passwd = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RegisterUserRequest {
    return {
      name: isSet(object.name) ? globalThis.String(object.name) : "",
      username: isSet(object.username) ? globalThis.String(object.username) : "",
      mobPhone: isSet(object.mobPhone) ? globalThis.String(object.mobPhone) : "",
      passwd: isSet(object.passwd) ? globalThis.String(object.passwd) : "",
    };
  },

  toJSON(message: RegisterUserRequest): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.username !== "") {
      obj.username = message.username;
    }
    if (message.mobPhone !== "") {
      obj.mobPhone = message.mobPhone;
    }
    if (message.passwd !== "") {
      obj.passwd = message.passwd;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RegisterUserRequest>, I>>(base?: I): RegisterUserRequest {
    return RegisterUserRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RegisterUserRequest>, I>>(object: I): RegisterUserRequest {
    const message = createBaseRegisterUserRequest();
    message.name = object.name ?? "";
    message.username = object.username ?? "";
    message.mobPhone = object.mobPhone ?? "";
    message.passwd = object.passwd ?? "";
    return message;
  },
};

function createBaseRegisterUserResponse(): RegisterUserResponse {
  return { name: "", message: "" };
}

export const RegisterUserResponse = {
  encode(message: RegisterUserResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.message !== "") {
      writer.uint32(18).string(message.message);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RegisterUserResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRegisterUserResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.message = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): RegisterUserResponse {
    return {
      name: isSet(object.name) ? globalThis.String(object.name) : "",
      message: isSet(object.message) ? globalThis.String(object.message) : "",
    };
  },

  toJSON(message: RegisterUserResponse): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.message !== "") {
      obj.message = message.message;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<RegisterUserResponse>, I>>(base?: I): RegisterUserResponse {
    return RegisterUserResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<RegisterUserResponse>, I>>(object: I): RegisterUserResponse {
    const message = createBaseRegisterUserResponse();
    message.name = object.name ?? "";
    message.message = object.message ?? "";
    return message;
  },
};

function createBaseSingleUserRetrieve(): SingleUserRetrieve {
  return { userId: "", name: "", username: "", mobPhone: "", accessLevel: "", status: "", aCreated: "" };
}

export const SingleUserRetrieve = {
  encode(message: SingleUserRetrieve, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== "") {
      writer.uint32(10).string(message.userId);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.username !== "") {
      writer.uint32(26).string(message.username);
    }
    if (message.mobPhone !== "") {
      writer.uint32(34).string(message.mobPhone);
    }
    if (message.accessLevel !== "") {
      writer.uint32(42).string(message.accessLevel);
    }
    if (message.status !== "") {
      writer.uint32(50).string(message.status);
    }
    if (message.aCreated !== "") {
      writer.uint32(58).string(message.aCreated);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SingleUserRetrieve {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSingleUserRetrieve();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.userId = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.name = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.username = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.mobPhone = reader.string();
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.accessLevel = reader.string();
          continue;
        case 6:
          if (tag !== 50) {
            break;
          }

          message.status = reader.string();
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          message.aCreated = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SingleUserRetrieve {
    return {
      userId: isSet(object.userId) ? globalThis.String(object.userId) : "",
      name: isSet(object.name) ? globalThis.String(object.name) : "",
      username: isSet(object.username) ? globalThis.String(object.username) : "",
      mobPhone: isSet(object.mobPhone) ? globalThis.String(object.mobPhone) : "",
      accessLevel: isSet(object.accessLevel) ? globalThis.String(object.accessLevel) : "",
      status: isSet(object.status) ? globalThis.String(object.status) : "",
      aCreated: isSet(object.aCreated) ? globalThis.String(object.aCreated) : "",
    };
  },

  toJSON(message: SingleUserRetrieve): unknown {
    const obj: any = {};
    if (message.userId !== "") {
      obj.userId = message.userId;
    }
    if (message.name !== "") {
      obj.name = message.name;
    }
    if (message.username !== "") {
      obj.username = message.username;
    }
    if (message.mobPhone !== "") {
      obj.mobPhone = message.mobPhone;
    }
    if (message.accessLevel !== "") {
      obj.accessLevel = message.accessLevel;
    }
    if (message.status !== "") {
      obj.status = message.status;
    }
    if (message.aCreated !== "") {
      obj.aCreated = message.aCreated;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SingleUserRetrieve>, I>>(base?: I): SingleUserRetrieve {
    return SingleUserRetrieve.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SingleUserRetrieve>, I>>(object: I): SingleUserRetrieve {
    const message = createBaseSingleUserRetrieve();
    message.userId = object.userId ?? "";
    message.name = object.name ?? "";
    message.username = object.username ?? "";
    message.mobPhone = object.mobPhone ?? "";
    message.accessLevel = object.accessLevel ?? "";
    message.status = object.status ?? "";
    message.aCreated = object.aCreated ?? "";
    return message;
  },
};

function createBaseAllUserRetrieve(): AllUserRetrieve {
  return { users: [] };
}

export const AllUserRetrieve = {
  encode(message: AllUserRetrieve, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    for (const v of message.users) {
      SingleUserRetrieve.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AllUserRetrieve {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAllUserRetrieve();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.users.push(SingleUserRetrieve.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): AllUserRetrieve {
    return {
      users: globalThis.Array.isArray(object?.users)
        ? object.users.map((e: any) => SingleUserRetrieve.fromJSON(e))
        : [],
    };
  },

  toJSON(message: AllUserRetrieve): unknown {
    const obj: any = {};
    if (message.users?.length) {
      obj.users = message.users.map((e) => SingleUserRetrieve.toJSON(e));
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<AllUserRetrieve>, I>>(base?: I): AllUserRetrieve {
    return AllUserRetrieve.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<AllUserRetrieve>, I>>(object: I): AllUserRetrieve {
    const message = createBaseAllUserRetrieve();
    message.users = object.users?.map((e) => SingleUserRetrieve.fromPartial(e)) || [];
    return message;
  },
};

function createBaseLoginRequest(): LoginRequest {
  return { username: "", password: "" };
}

export const LoginRequest = {
  encode(message: LoginRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.username !== "") {
      writer.uint32(10).string(message.username);
    }
    if (message.password !== "") {
      writer.uint32(18).string(message.password);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LoginRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLoginRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.username = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.password = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): LoginRequest {
    return {
      username: isSet(object.username) ? globalThis.String(object.username) : "",
      password: isSet(object.password) ? globalThis.String(object.password) : "",
    };
  },

  toJSON(message: LoginRequest): unknown {
    const obj: any = {};
    if (message.username !== "") {
      obj.username = message.username;
    }
    if (message.password !== "") {
      obj.password = message.password;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<LoginRequest>, I>>(base?: I): LoginRequest {
    return LoginRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<LoginRequest>, I>>(object: I): LoginRequest {
    const message = createBaseLoginRequest();
    message.username = object.username ?? "";
    message.password = object.password ?? "";
    return message;
  },
};

function createBaseGetUserRequest(): GetUserRequest {
  return { userId: 0 };
}

export const GetUserRequest = {
  encode(message: GetUserRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== 0) {
      writer.uint32(8).int32(message.userId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetUserRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetUserRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.userId = reader.int32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): GetUserRequest {
    return { userId: isSet(object.userId) ? globalThis.Number(object.userId) : 0 };
  },

  toJSON(message: GetUserRequest): unknown {
    const obj: any = {};
    if (message.userId !== 0) {
      obj.userId = Math.round(message.userId);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<GetUserRequest>, I>>(base?: I): GetUserRequest {
    return GetUserRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<GetUserRequest>, I>>(object: I): GetUserRequest {
    const message = createBaseGetUserRequest();
    message.userId = object.userId ?? 0;
    return message;
  },
};

function createBaseDeleteUserRequest(): DeleteUserRequest {
  return { userId: 0 };
}

export const DeleteUserRequest = {
  encode(message: DeleteUserRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== 0) {
      writer.uint32(8).int32(message.userId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeleteUserRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteUserRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.userId = reader.int32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): DeleteUserRequest {
    return { userId: isSet(object.userId) ? globalThis.Number(object.userId) : 0 };
  },

  toJSON(message: DeleteUserRequest): unknown {
    const obj: any = {};
    if (message.userId !== 0) {
      obj.userId = Math.round(message.userId);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<DeleteUserRequest>, I>>(base?: I): DeleteUserRequest {
    return DeleteUserRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<DeleteUserRequest>, I>>(object: I): DeleteUserRequest {
    const message = createBaseDeleteUserRequest();
    message.userId = object.userId ?? 0;
    return message;
  },
};

function createBaseChangePasswordRequest(): ChangePasswordRequest {
  return { userId: 0, oldPassword: "", newPassword: "" };
}

export const ChangePasswordRequest = {
  encode(message: ChangePasswordRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== 0) {
      writer.uint32(8).int32(message.userId);
    }
    if (message.oldPassword !== "") {
      writer.uint32(18).string(message.oldPassword);
    }
    if (message.newPassword !== "") {
      writer.uint32(26).string(message.newPassword);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ChangePasswordRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseChangePasswordRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.userId = reader.int32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.oldPassword = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.newPassword = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ChangePasswordRequest {
    return {
      userId: isSet(object.userId) ? globalThis.Number(object.userId) : 0,
      oldPassword: isSet(object.oldPassword) ? globalThis.String(object.oldPassword) : "",
      newPassword: isSet(object.newPassword) ? globalThis.String(object.newPassword) : "",
    };
  },

  toJSON(message: ChangePasswordRequest): unknown {
    const obj: any = {};
    if (message.userId !== 0) {
      obj.userId = Math.round(message.userId);
    }
    if (message.oldPassword !== "") {
      obj.oldPassword = message.oldPassword;
    }
    if (message.newPassword !== "") {
      obj.newPassword = message.newPassword;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ChangePasswordRequest>, I>>(base?: I): ChangePasswordRequest {
    return ChangePasswordRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ChangePasswordRequest>, I>>(object: I): ChangePasswordRequest {
    const message = createBaseChangePasswordRequest();
    message.userId = object.userId ?? 0;
    message.oldPassword = object.oldPassword ?? "";
    message.newPassword = object.newPassword ?? "";
    return message;
  },
};

function createBaseAssignRoleRequest(): AssignRoleRequest {
  return { userId: 0, newRole: "" };
}

export const AssignRoleRequest = {
  encode(message: AssignRoleRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== 0) {
      writer.uint32(8).int32(message.userId);
    }
    if (message.newRole !== "") {
      writer.uint32(18).string(message.newRole);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AssignRoleRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAssignRoleRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.userId = reader.int32();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.newRole = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): AssignRoleRequest {
    return {
      userId: isSet(object.userId) ? globalThis.Number(object.userId) : 0,
      newRole: isSet(object.newRole) ? globalThis.String(object.newRole) : "",
    };
  },

  toJSON(message: AssignRoleRequest): unknown {
    const obj: any = {};
    if (message.userId !== 0) {
      obj.userId = Math.round(message.userId);
    }
    if (message.newRole !== "") {
      obj.newRole = message.newRole;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<AssignRoleRequest>, I>>(base?: I): AssignRoleRequest {
    return AssignRoleRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<AssignRoleRequest>, I>>(object: I): AssignRoleRequest {
    const message = createBaseAssignRoleRequest();
    message.userId = object.userId ?? 0;
    message.newRole = object.newRole ?? "";
    return message;
  },
};

function createBaseSuspendAccountRequest(): SuspendAccountRequest {
  return { userId: 0 };
}

export const SuspendAccountRequest = {
  encode(message: SuspendAccountRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.userId !== 0) {
      writer.uint32(8).int32(message.userId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SuspendAccountRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSuspendAccountRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.userId = reader.int32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SuspendAccountRequest {
    return { userId: isSet(object.userId) ? globalThis.Number(object.userId) : 0 };
  },

  toJSON(message: SuspendAccountRequest): unknown {
    const obj: any = {};
    if (message.userId !== 0) {
      obj.userId = Math.round(message.userId);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SuspendAccountRequest>, I>>(base?: I): SuspendAccountRequest {
    return SuspendAccountRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SuspendAccountRequest>, I>>(object: I): SuspendAccountRequest {
    const message = createBaseSuspendAccountRequest();
    message.userId = object.userId ?? 0;
    return message;
  },
};

function createBaseEmptyReqRes(): EmptyReqRes {
  return {};
}

export const EmptyReqRes = {
  encode(_: EmptyReqRes, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EmptyReqRes {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEmptyReqRes();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): EmptyReqRes {
    return {};
  },

  toJSON(_: EmptyReqRes): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<EmptyReqRes>, I>>(base?: I): EmptyReqRes {
    return EmptyReqRes.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<EmptyReqRes>, I>>(_: I): EmptyReqRes {
    const message = createBaseEmptyReqRes();
    return message;
  },
};

export interface StaffUsers {
  RegisterUser(request: DeepPartial<RegisterUserRequest>, metadata?: grpc.Metadata): Promise<RegisterUserResponse>;
  Login(request: DeepPartial<LoginRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve>;
  GetUser(request: DeepPartial<GetUserRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve>;
  GetAllUsers(request: DeepPartial<EmptyReqRes>, metadata?: grpc.Metadata): Promise<AllUserRetrieve>;
  DeleteUser(request: DeepPartial<DeleteUserRequest>, metadata?: grpc.Metadata): Promise<EmptyReqRes>;
  EditUser(request: DeepPartial<SingleUserRetrieve>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve>;
  ChangePassword(request: DeepPartial<ChangePasswordRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve>;
  AssignRole(request: DeepPartial<AssignRoleRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve>;
  SuspendAccount(request: DeepPartial<SuspendAccountRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve>;
}

export class StaffUsersClientImpl implements StaffUsers {
  private readonly rpc: Rpc;

  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.RegisterUser = this.RegisterUser.bind(this);
    this.Login = this.Login.bind(this);
    this.GetUser = this.GetUser.bind(this);
    this.GetAllUsers = this.GetAllUsers.bind(this);
    this.DeleteUser = this.DeleteUser.bind(this);
    this.EditUser = this.EditUser.bind(this);
    this.ChangePassword = this.ChangePassword.bind(this);
    this.AssignRole = this.AssignRole.bind(this);
    this.SuspendAccount = this.SuspendAccount.bind(this);
  }

  RegisterUser(request: DeepPartial<RegisterUserRequest>, metadata?: grpc.Metadata): Promise<RegisterUserResponse> {
    return this.rpc.unary(StaffUsersRegisterUserDesc, RegisterUserRequest.fromPartial(request), metadata);
  }

  Login(request: DeepPartial<LoginRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve> {
    return this.rpc.unary(StaffUsersLoginDesc, LoginRequest.fromPartial(request), metadata);
  }

  GetUser(request: DeepPartial<GetUserRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve> {
    return this.rpc.unary(StaffUsersGetUserDesc, GetUserRequest.fromPartial(request), metadata);
  }

  GetAllUsers(request: DeepPartial<EmptyReqRes>, metadata?: grpc.Metadata): Promise<AllUserRetrieve> {
    return this.rpc.unary(StaffUsersGetAllUsersDesc, EmptyReqRes.fromPartial(request), metadata);
  }

  DeleteUser(request: DeepPartial<DeleteUserRequest>, metadata?: grpc.Metadata): Promise<EmptyReqRes> {
    return this.rpc.unary(StaffUsersDeleteUserDesc, DeleteUserRequest.fromPartial(request), metadata);
  }

  EditUser(request: DeepPartial<SingleUserRetrieve>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve> {
    return this.rpc.unary(StaffUsersEditUserDesc, SingleUserRetrieve.fromPartial(request), metadata);
  }

  ChangePassword(request: DeepPartial<ChangePasswordRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve> {
    return this.rpc.unary(StaffUsersChangePasswordDesc, ChangePasswordRequest.fromPartial(request), metadata);
  }

  AssignRole(request: DeepPartial<AssignRoleRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve> {
    return this.rpc.unary(StaffUsersAssignRoleDesc, AssignRoleRequest.fromPartial(request), metadata);
  }

  SuspendAccount(request: DeepPartial<SuspendAccountRequest>, metadata?: grpc.Metadata): Promise<SingleUserRetrieve> {
    return this.rpc.unary(StaffUsersSuspendAccountDesc, SuspendAccountRequest.fromPartial(request), metadata);
  }
}

export const StaffUsersDesc = { serviceName: "staffusers.StaffUsers" };

export const StaffUsersRegisterUserDesc: UnaryMethodDefinitionish = {
  methodName: "RegisterUser",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return RegisterUserRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = RegisterUserResponse.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const StaffUsersLoginDesc: UnaryMethodDefinitionish = {
  methodName: "Login",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return LoginRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = SingleUserRetrieve.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const StaffUsersGetUserDesc: UnaryMethodDefinitionish = {
  methodName: "GetUser",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return GetUserRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = SingleUserRetrieve.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const StaffUsersGetAllUsersDesc: UnaryMethodDefinitionish = {
  methodName: "GetAllUsers",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return EmptyReqRes.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = AllUserRetrieve.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const StaffUsersDeleteUserDesc: UnaryMethodDefinitionish = {
  methodName: "DeleteUser",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return DeleteUserRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = EmptyReqRes.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const StaffUsersEditUserDesc: UnaryMethodDefinitionish = {
  methodName: "EditUser",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return SingleUserRetrieve.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = SingleUserRetrieve.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const StaffUsersChangePasswordDesc: UnaryMethodDefinitionish = {
  methodName: "ChangePassword",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return ChangePasswordRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = SingleUserRetrieve.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const StaffUsersAssignRoleDesc: UnaryMethodDefinitionish = {
  methodName: "AssignRole",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return AssignRoleRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = SingleUserRetrieve.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

export const StaffUsersSuspendAccountDesc: UnaryMethodDefinitionish = {
  methodName: "SuspendAccount",
  service: StaffUsersDesc,
  requestStream: false,
  responseStream: false,
  requestType: {
    serializeBinary() {
      return SuspendAccountRequest.encode(this).finish();
    },
  } as any,
  responseType: {
    deserializeBinary(data: Uint8Array) {
      const value = SingleUserRetrieve.decode(data);
      return {
        ...value,
        toObject() {
          return value;
        },
      };
    },
  } as any,
};

interface UnaryMethodDefinitionishR extends grpc.UnaryMethodDefinition<any, any> {
  requestStream: any;
  responseStream: any;
}

type UnaryMethodDefinitionish = UnaryMethodDefinitionishR;

interface Rpc {
  unary<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    request: any,
    metadata: grpc.Metadata | undefined,
  ): Promise<any>;
}

export class GrpcWebImpl {
  private host: string;
  private options: {
    transport?: grpc.TransportFactory;

    debug?: boolean;
    metadata?: grpc.Metadata;
    upStreamRetryCodes?: number[];
  };

  constructor(
    host: string,
    options: {
      transport?: grpc.TransportFactory;

      debug?: boolean;
      metadata?: grpc.Metadata;
      upStreamRetryCodes?: number[];
    },
  ) {
    this.host = host;
    this.options = options;
  }

  unary<T extends UnaryMethodDefinitionish>(
    methodDesc: T,
    _request: any,
    metadata: grpc.Metadata | undefined,
  ): Promise<any> {
    const request = { ..._request, ...methodDesc.requestType };
    const maybeCombinedMetadata = metadata && this.options.metadata
      ? new BrowserHeaders({ ...this.options?.metadata.headersMap, ...metadata?.headersMap })
      : metadata ?? this.options.metadata;
    return new Promise((resolve, reject) => {
      grpc.unary(methodDesc, {
        request,
        host: this.host,
        metadata: maybeCombinedMetadata ?? {},
        ...(this.options.transport !== undefined ? { transport: this.options.transport } : {}),
        debug: this.options.debug ?? false,
        onEnd: function (response) {
          if (response.status === grpc.Code.OK) {
            resolve(response.message!.toObject());
          } else {
            const err = new GrpcWebError(response.statusMessage, response.status, response.trailers);
            reject(err);
          }
        },
      });
    });
  }
}

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}

export class GrpcWebError extends globalThis.Error {
  constructor(message: string, public code: grpc.Code, public metadata: grpc.Metadata) {
    super(message);
  }
}
