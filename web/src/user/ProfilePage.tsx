import { type User, useDeleteUser, useUpdateUser } from "@api/client";
import { useForm } from "@tanstack/react-form";
import { useNavigate } from "@tanstack/react-router";
import { useAuthContext } from "src/auth/AuthContext";

interface ProfilePageProps {
  user: User;
}

const ProfilePage = ({ user }: ProfilePageProps) => {
  return (
    <>
      <div>
        Hello {user.first_name} {user.last_name} from a!
      </div>
      <ProfileForm user={user} />
      <DeleteUserButton user={user} />
    </>
  );
};

interface ProfileFormProps {
  user: User;
}

const ProfileForm = ({ user }: ProfileFormProps) => {
  const mutation = useUpdateUser();

  const form = useForm({
    defaultValues: {
      first_name: user.first_name,
      last_name: user.last_name,
    },
    onSubmit: ({ value }) => mutation.mutateAsync({ id: user.id, value }),
  });

  return (
    <div>
      Profile
      <form
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
      >
        <form.Field
          name="first_name"
          validators={{
            onChange: ({ value }) =>
              !value ? "A first name is required" : undefined,
          }}
        >
          {(field) => (
            <div>
              <label htmlFor={field.name}>First name:</label>
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
              <label htmlFor={field.name}>Last name:</label>
              <input
                id={field.name}
                type="last_name"
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

interface DeleteUserButtonProps {
  user: User;
}

const DeleteUserButton = ({ user }: DeleteUserButtonProps) => {
  const { logout } = useAuthContext();
  const mutation = useDeleteUser();
  const navigate = useNavigate();

  return (
    <button
      type="button"
      onClick={async () => {
        await mutation.mutateAsync(user.id);
        logout();
        navigate({ to: "/" });
      }}
    >
      Delete Account
    </button>
  );
};

export { ProfilePage };
