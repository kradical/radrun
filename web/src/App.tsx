import { useCreateAccount } from "./api/client";

const App = () => {
  const mutation = useCreateAccount();
  return (
    <div>
      We r so in
      <button
        type="button"
        onClick={() =>
          mutation.mutate({
            email: "bloop123",
            first_name: "bloop",
            last_name: "bloop",
            password: "bloop2",
          })
        }
      >
        CLICK
      </button>
    </div>
  );
};

export { App };
