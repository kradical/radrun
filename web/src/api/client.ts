import { useMutation } from "@tanstack/react-query";
import type { AccountRes, CreateAccountReq } from "./generated/account";

const parseRes = <T>(res: Response): Promise<T> => {
  if (res.status >= 400) {
    throw new Error(`${res.status}: ${res.url} request failed`);
  }

  return res.json();
};

type Account = Omit<AccountRes, "created_at" | "updated_at"> & {
  created_at: Date;
  updated_at: Date;
};

const createAccount = (req: CreateAccountReq): Promise<Account> =>
  fetch("/account", { method: "POST", body: JSON.stringify(req) })
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

export { type Account, useCreateAccount };
