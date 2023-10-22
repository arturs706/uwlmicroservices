import { Elysia, NotFoundError } from "elysia";
import { jwt } from '@elysiajs/jwt'



const app = new Elysia()
.use(
  jwt({
    secret: Bun.env.JWT_SECRET as string,
  })
)
.get('/', async ({ jwt, request, set }) => {
  const headers = request.headers;
   const authorizationHeader = headers.get('authorization');
   if (authorizationHeader) {
    const token = authorizationHeader.split(' ')[1];

    const payload = await jwt.verify(token);
    if (typeof payload === 'object' && payload !== null) {
      const isExpired = payload.exp
        ? Date.now() > payload.exp * 1000
        : false;
        console.log(isExpired)
      if ((payload.iss === 'gateway' ) && ((Boolean(payload.gateway_access)) === true) && !isExpired) {
        set.status = 200
        return 'Hello world'
      }
    }
    // set.status = 404
    // return 'Not found'
    //NotFoundError 
    throw new NotFoundError('Not found')
  } 

  throw new NotFoundError('Not found')
})
.listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
