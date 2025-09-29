import { useLogin } from "@api/client";
import type { LoginReq } from "@api/generated/auth";
import { useForm } from "@tanstack/react-form";
import { useRouter, useSearch } from "@tanstack/react-router";
import { useSetSessionExpires } from "./AuthContext";

const defaultValues: LoginReq = {
  email: "",
  password: "",
};

const LoginPage = () => {
  const setSessionExpires = useSetSessionExpires();
  const mutation = useLogin();

  const router = useRouter();
  const search = useSearch({
    from: "/login",
  });

  const form = useForm({
    defaultValues,
    onSubmit: async ({ value }) => {
      const res = await mutation.mutateAsync(value);
      setSessionExpires(res.expires_at);
      router.history.push(search.redirect ?? "/");
    },
  });

  return (
    <div>
      Login
      <form
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
      >
        <form.Field
          name="email"
          validators={{
            onChange: ({ value }) =>
              !value ? "An email is required" : undefined,
          }}
        >
          {(field) => (
            <div>
              <label htmlFor={field.name}>Email:</label>
              <input
                id={field.name}
                required
                name={field.name}
                value={field.state.value}
                onBlur={field.handleBlur}
                onChange={(e) => field.handleChange(e.target.value)}
              />
            </div>
          )}
        </form.Field>
        <form.Field
          name="password"
          validators={{
            onChange: ({ value }) =>
              !value ? "A password is required" : undefined,
          }}
        >
          {(field) => (
            <div>
              <label htmlFor={field.name}>Password:</label>
              <input
                id={field.name}
                type="password"
                required
                name={field.name}
                value={field.state.value}
                onBlur={field.handleBlur}
                onChange={(e) => field.handleChange(e.target.value)}
              />
            </div>
          )}
        </form.Field>
        <form.Subscribe
          selector={(state) => [state.canSubmit, state.isSubmitting]}
        >
          {([canSubmit, isSubmitting]) => (
            <button type="submit" disabled={!canSubmit}>
              {isSubmitting ? "..." : "Submit"}
            </button>
          )}
        </form.Subscribe>
      </form>
    </div>
  );
};

export { LoginPage };
