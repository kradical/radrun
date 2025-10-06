import { queryOptions, useMutation } from "@tanstack/react-query";
import type {
  LoginReq,
  LoginRes,
  SignUpReq,
  SignUpRes,
} from "./generated/auth";
import type { UserRes } from "./generated/user";

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

type Session = Omit<LoginRes, "expires_at"> & {
  expires_at: Date;
};

const login = (req: LoginReq): Promise<Session> =>
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

type SignUp = {
  user: User;
  session: Session;
};

const signUp = (req: SignUpReq): Promise<SignUp> =>
  fetch("/api/auth/sign-up", {
    method: "POST",
    headers: { [contentTypeHeader]: jsonContentType },
    body: JSON.stringify(req),
  })
    .then(parseRes<SignUpRes>)
    .then((res) => ({
      ...res,
      user: {
        ...res.user,
        created_at: new Date(res.user.created_at),
        updated_at: new Date(res.user.updated_at),
      },
      session: {
        ...res.session,
        expires_at: new Date(res.session.expires_at),
      },
    }));

const useSignUp = () => useMutation({ mutationFn: signUp });

// User

type User = Omit<UserRes, "created_at" | "updated_at"> & {
  created_at: Date;
  updated_at: Date;
};

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

export { type User, useSignUp, useLogin, currentUserQueryOptions };
