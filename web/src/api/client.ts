import { useMutation } from "@tanstack/react-query";
import type { AccountCreateReq, AccountRes } from "./generated/account";
import type { LoginReq, LoginRes } from "./generated/auth";

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

const login = (req: LoginReq): Promise<LoginRes> =>
  fetch("/api/auth/login", {
    method: "POST",
    headers: { [contentTypeHeader]: jsonContentType },
    body: JSON.stringify(req),
  }).then(parseRes<LoginRes>);

const useLogin = () =>
  useMutation({
    mutationFn: login,
    onSuccess: (res) => console.log(res),
  });

// Account

type Account = Omit<AccountRes, "created_at" | "updated_at"> & {
  created_at: Date;
  updated_at: Date;
};

const createAccount = (req: AccountCreateReq): Promise<Account> =>
  fetch("/api/account", {
    method: "POST",
    headers: { [contentTypeHeader]: jsonContentType },
    body: JSON.stringify(req),
  })
    .then(parseRes<AccountRes>)
    .then((res) => ({
      ...res,
      created_at: new Date(res.created_at),
      updated_at: new Date(res.updated_at),
    }));

const useCreateAccount = () =>
  useMutation({
    mutationFn: createAccount,
    onSuccess: (res) => console.log(res),
  });

export { type Account, useCreateAccount, useLogin };
