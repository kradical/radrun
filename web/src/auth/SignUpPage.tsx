import { useSignUp } from "@api/client";
import type { SignUpReq } from "@api/generated/auth";
import { useForm } from "@tanstack/react-form";
import { useRouter, useSearch } from "@tanstack/react-router";
import { useAuthContext } from "./AuthContext";

const defaultValues: SignUpReq = {
  first_name: "",
  last_name: "",
  email: "",
  password: "",
};

const SignUpPage = () => {
  const mutation = useSignUp();
  const { setSessionExpires } = useAuthContext();

  const router = useRouter();
  const search = useSearch({
    from: "/sign-up",
  });

  const form = useForm({
    defaultValues,
    onSubmit: async ({ value }) => {
      const res = await mutation.mutateAsync(value);
      setSessionExpires(res.session.expires_at);
      router.history.push(search.redirect ?? "/");
    },
  });

  return (
    <div>
      Sign Up
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
        <form.Field
          name="first_name"
          validators={{
            onChange: ({ value }) =>
              !value ? "A first name is required" : undefined,
          }}
        >
          {(field) => (
            <div>
              <label htmlFor={field.name}>First Name:</label>
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
          name="last_name"
          validators={{
            onChange: ({ value }) =>
              !value ? "A last name is required" : undefined,
          }}
        >
          {(field) => (
            <div>
              <label htmlFor={field.name}>Last Name:</label>
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

export { SignUpPage };
