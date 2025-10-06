import { queryOptions, useMutation, useQuery } from "@tanstack/react-query";
import type { LoginReq, LoginRes } from "./generated/auth";
import type { UserCreateReq, UserRes } from "./generated/user";

// Shared

const parseRes = <T>(res: Response): Promise<T> => {
  if (res.status >= 400) {
    throw new Error(`${res.status}: ${res.url} request failed`);
  }

  return res.json();
};

const contentTypeHeader = "Content-Type";
const jsonContentType = "application/json";

// Auth

type Login = Omit<LoginRes, "expires_at"> & {
  expires_at: Date;
};

const login = (req: LoginReq): Promise<Login> =>
  fetch("/api/auth/login", {
    method: "POST",
    headers: { [contentTypeHeader]: jsonContentType },
    body: JSON.stringify(req),
  })
    .then(parseRes<LoginRes>)
    .then((res) => ({
      ...res,
      expires_at: new Date(res.expires_at),
    }));

const useLogin = () =>
  useMutation({
    mutationFn: login,
    onSuccess: (res) => console.log(res),
  });

// User

type User = Omit<UserRes, "created_at" | "updated_at"> & {
  created_at: Date;
  updated_at: Date;
};

const createUser = (req: UserCreateReq): Promise<User> =>
  fetch("/api/user", {
    method: "POST",
    headers: { [contentTypeHeader]: jsonContentType },
    body: JSON.stringify(req),
  })
    .then(parseRes<UserRes>)
    .then((res) => ({
      ...res,
      created_at: new Date(res.created_at),
      updated_at: new Date(res.updated_at),
    }));

const useCreateUser = () =>
  useMutation({
    mutationFn: createUser,
    onSuccess: (res) => console.log(res),
  });

const getCurrentUser = (): Promise<User> =>
  fetch("/api/auth/me", {
    method: "GET",
    headers: { [contentTypeHeader]: jsonContentType },
  })
    .then(parseRes<UserRes>)
    .then((res) => ({
      ...res,
      created_at: new Date(res.created_at),
      updated_at: new Date(res.updated_at),
    }));

// Not 100% sure how I want to go about exposing this vs. hooks / etc.
const currentUserQueryOptions = queryOptions({
  queryKey: ["user", "me"],
  queryFn: getCurrentUser,
});

export { type User, useCreateUser, useLogin, currentUserQueryOptions };
